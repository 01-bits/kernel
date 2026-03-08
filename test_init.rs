// Test to understand the GDT init issue
// The problem: options(nostack) but using push/retfq

// When abi_x86_interrupt is enabled, it changes calling conventions
// which might affect stack alignment or how registers are saved
