//! Draws the plug-in's real scene with the player's real renderer.

use popamp_plugin_spectrum::scene_json;
use popamp_visual::{wgpu, Frame, Gpu, Renderer, Scene, SPECTRUM_LEN, WAVE_LEN};

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const SIZE: u32 = 256;
const BYTES_PER_ROW: u32 = SIZE * 4;
const DT: f32 = 1.0 / 60.0;
const WAVE: [f32; WAVE_LEN] = [0.0; WAVE_LEN];
const SILENCE: [f32; SPECTRUM_LEN] = [0.0; SPECTRUM_LEN];
const HALF: [f32; SPECTRUM_LEN] = [0.5; SPECTRUM_LEN];
/// In the middle of a bar, inside the lower half of the window.
const IN_A_BAR: (u32, u32) = (130, 200);

struct Stage {
    gpu: Gpu,
    renderer: Renderer,
    output: wgpu::Texture,
}

impl Stage {
    /// No GPU at all (a bare build machine) skips the test with a note.
    fn new() -> Option<Stage> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let gpu = match Gpu::new(&instance, None) {
            Ok(gpu) => gpu,
            Err(e) => {
                eprintln!("skipped: {e}");
                return None;
            }
        };
        let scene = Scene::parse(scene_json().as_bytes()).unwrap();
        let renderer = Renderer::new(&gpu, scene, FORMAT, (SIZE, SIZE)).unwrap();
        let output = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: extent(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        Some(Stage { gpu, renderer, output })
    }

    fn draw(&mut self, spectrum: &[f32; SPECTRUM_LEN], frames: usize) {
        let view = self.output.create_view(&Default::default());
        for _ in 0..frames {
            let frame = Frame { time: 1.0, dt: DT, wave: &WAVE, spectrum, data: &[] };
            self.renderer.draw(&self.gpu, &view, &frame).unwrap();
        }
    }

    /// The brightest colour channel of a pixel.
    fn brightness(&self, (x, y): (u32, u32)) -> u8 {
        let buffer = self.gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(BYTES_PER_ROW * SIZE),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        let layout = wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(BYTES_PER_ROW), rows_per_image: None };
        encoder.copy_texture_to_buffer(self.output.as_image_copy(), wgpu::TexelCopyBufferInfo { buffer: &buffer, layout }, extent());
        self.gpu.queue.submit([encoder.finish()]);
        buffer.slice(..).map_async(wgpu::MapMode::Read, |r| r.unwrap());
        self.gpu.device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        let bytes = buffer.slice(..).get_mapped_range().unwrap();
        let at = (y * BYTES_PER_ROW + x * 4) as usize;
        bytes[at..at + 3].iter().copied().max().unwrap()
    }
}

fn extent() -> wgpu::Extent3d {
    wgpu::Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: 1 }
}

#[test]
fn the_player_accepts_the_scene() {
    Scene::parse(scene_json().as_bytes()).unwrap();
}

#[test]
fn silence_draws_nothing() {
    let Some(mut stage) = Stage::new() else { return };
    stage.draw(&SILENCE, 3);
    assert_eq!(stage.brightness(IN_A_BAR), 0);
}

#[test]
fn sound_lights_its_bar_up_to_its_level() {
    let Some(mut stage) = Stage::new() else { return };
    stage.draw(&HALF, 1);
    assert!(stage.brightness(IN_A_BAR) > 100, "the bar is dark");
    assert_eq!(stage.brightness((130, 60)), 0, "the upper half should be empty at level 0.5");
}

#[test]
fn a_bar_leaves_a_trail_that_dies_away() {
    let Some(mut stage) = Stage::new() else { return };
    stage.draw(&HALF, 1);
    let lit = stage.brightness(IN_A_BAR);
    stage.draw(&SILENCE, 1);
    let trail = stage.brightness(IN_A_BAR);
    assert!(trail > 0 && trail < lit, "after one silent frame: {trail}, the bar was {lit}");
    stage.draw(&SILENCE, 300);
    assert_eq!(stage.brightness(IN_A_BAR), 0, "still lit after 5 s of silence");
}
