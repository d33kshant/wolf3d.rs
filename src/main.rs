use console_engine::*;

///3.14159265358979323846264338327950288
const PI: f64 = std::f64::consts::PI;

/// ### Camera
///	A camera with position (x, y)
/// and some angle in redian with some field of view and maximum
/// reder depth
struct Camera {
	x: f64,
	y: f64,
	angle: f64,
	fov: f64,
	depth: f64,
}

impl Camera {
	/// Instansiate new camera with initial position (x, y) and some angle in redian
	fn new(x: f64, y: f64, angle: f64) -> Self {
		Camera { x, y, angle, fov: PI/3.0, depth: 16.0 }
	}

	/// move the camera in forward direction of its angle
	fn move_farward(&mut self) { 
		self.x += self.angle.sin();
		self.y += self.angle.cos();
	}
	
	/// move the camera in backward direction of its angle
	fn move_backward(&mut self) { 
		self.x -= self.angle.sin();
		self.y -= self.angle.cos();
	}

	/// Rotate camera 2deg in anti-clockwise direction
	fn rotate_left(&mut self) { self.angle -= PI/90.0; }
	
	/// Rotate camera 2deg in clockwise direction
	fn rotate_right(&mut self) { self.angle += PI/90.0; }

}

// Main Game
fn main() {

	if let Some((width, height)) = term_size::dimensions() {

		// Initialize an engine with WIDTH HEIGHT and FPS
		let mut engine = ConsoleEngine::init(width as u32, height as u32, 30).expect("Failed to initialize console.");

		// 1D vector of 8x8 element representing world map
		// where 1 is block and 0 is void
		let map: [u8; 64] = [
			1, 1, 1, 1, 1, 1, 1, 1,
			1, 0, 1, 0, 0, 0, 0, 1,
			1, 0, 0, 0, 0, 1, 0, 1,
			1, 0, 0, 0, 0, 1, 0, 1,
			1, 0, 0, 0, 0, 0, 0, 1,
			1, 0, 0, 0, 1, 1, 1, 1,
			1, 1, 0, 0, 0, 0, 0, 1,
			1, 1, 1, 1, 1, 1, 1, 1,
		];

		// Initialize camera at position (0, 0) with angle 0rad
		let mut camera = Camera::new(3.0, 3.0, 0.0);
		
		// The main game loop
		loop {
			
			// Break loop on escape key press
			if engine.is_key_pressed(KeyCode::Esc) { break; }

			// Rotate camera based on input
			if engine.is_key_held(KeyCode::Char('a')) { camera.rotate_left() }
			if engine.is_key_held(KeyCode::Char('d')) { camera.rotate_right() }

			// Move the camera based on input
			if engine.is_key_pressed(KeyCode::Char('w')) { camera.move_farward() }
			if engine.is_key_pressed(KeyCode::Char('s')) { camera.move_backward() }

			// Pause the execution until the next frame need to be rendered
			// Clear the screen in order to redraw console
			engine.wait_frame();
			engine.clear_screen();

			// For every column in window calculate a wall distance
			for x in 0..width {

				// Calculate ray angle based on camera angle
				let ray_angle: f64 = (camera.angle - camera.fov / 2.0) + (x as f64 / width as f64) * camera.fov;
				let ray_sin = ray_angle.sin();	// Sin of camera angle
				let ray_cos = ray_angle.cos();	// Cos of camera angle

				// Initialize distance to 0.0
				let mut distance: f64 = 0.0;
				loop {

					// Increment small amount in every loop for raycasting
					distance += 0.1;

					// Calculate the point in ray_angle to calculate collision
					let ray_x = camera.x + ray_sin * distance;
					let ray_y = camera.y + ray_cos * distance;

					// If ray is out of map set distance to max depth
					if ray_x < 0.0 || ray_y < 0.0 || ray_x >= 8.0 || ray_y >= 8.0 {
						distance = camera.depth;
						break;
					}
					
					// If the ray collide with a wall distance is calculated
					else {
						let index = ray_y as usize * 8 + ray_x as usize;
						if map[index] == 1 {
							break;
						}
					}
				}

				// Calculate start of ceil and floor based on distance value
				let ceil = height as f64 / 2.0 - height as f64 / distance;
				let floor = height as f64 - ceil;

				// For every column if a point is in range [ceil, floor]
				// draw a shade based on distance
				for y in 0..height {
					if y as f64 >= ceil && y as f64 <= floor {
						let pixel = 
							if distance <= 1.0 { "█" }
							else if distance <= 2.0 { "▓" }
							else if distance <= 3.0 { "▒" }
							else if distance <= 5.0 { "░" }
							else { " " };
						engine.print(x as i32, y as i32, pixel);
					}
				}
			}
			
			// Print the map and camera position
			// in top left corner of the screen
			for x in 0..8 {
				for y in 0..8 {
					let pixel =
						if x == camera.x as usize && y == camera.y as usize { "*" }
						else if map[y*8+x] == 1 { "#"}
						else { " " };
					engine.print(x as i32, y as i32, pixel);
				}
			}
			
			// Draw the current frame
			engine.draw();
		}	
	} else {
		println!("Failed to get the dimensions.")
	}
}
