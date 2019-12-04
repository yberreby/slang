	.globl	main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	movl	$24, %eax
	popq	%rbp
	ret
