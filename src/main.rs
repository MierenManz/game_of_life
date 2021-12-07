mod cell;
mod event_loop;

use cell::Grid;
use event_loop::ControlFlow;
use event_loop::EventLoop;

fn main() {
    let grid = Grid::new_randomized(12, 12);
    let event_loop = EventLoop::new(grid);
    event_loop.run(|grid, control_flow| {
        println!("{}", grid);
        if grid.compute_next_generation().is_err() {
            *control_flow = ControlFlow::Stop;
        }
        *control_flow = ControlFlow::Wait(500);
    });
}
