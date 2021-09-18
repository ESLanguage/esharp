;.data

;.text

;.global test_asm
test_asm:
	mov eax, 3 ; don't forget to use 32-bit registers as this is an i32!
	add eax, edi
	ret

;.global init
init:
	ret

step:

