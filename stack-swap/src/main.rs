use std::arch::asm; // Import for using inline assembly

const SSIZE: isize = 48; // Define the size of our thread stack

// A simplified struct to represent the essential context of a thread (just the stack pointer )
#[derive(Debug, Default)]
#[repr(C)] // Ensure C-compatible memory layout
struct ThreadContext {
    rsp: u64, // The 64-bit register holding the stack pointer
}

// Our thread function - it prints a message and enters an infinite loop (no yielding) 
fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");
    loop {} 
}

// This function is 'unsafe' as it directly manipulates the stack pointer using assembly
unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        // Inline assembly instructions:
        "mov rsp, [{0} + 0x00]", // Load the stack pointer (rsp) from the 'ThreadContext' pointed by 'new'
        "ret",                  // Return from the function, effectively switching stacks
        in(reg) new,            // 'new' pointer is our input operand 
    );
}

fn main() {
    // Create a default-initialized ThreadContext for our initial thread
    let mut ctx = ThreadContext::default();

    // Allocate a vector to act as our thread's stack 
    let mut stack = vec![0_u8; SSIZE as usize];

    unsafe { 
        // Calculate the usable starting point of our stack
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);

        // Align stack to 16-byte boundary (often better for assembly instructions)
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;

        // Simulate 'calling' the 'hello' function: place its address where a return address would be
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);

        // Set the initial stack pointer in our ThreadContext 
        ctx.rsp = sb_aligned.offset(-16) as u64; 

        // The magic happens! Switch context to the newly prepared stack
        gt_switch(&mut ctx); 
    }
}