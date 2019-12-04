# slang - A Simple Language

Slang aims to sit at a C level of abstraction.

Code generation target: x86 ASM (Intel syntax)


## Features

Instruction: `interrupt <interrupt code>`

Méchanisme pour accéder aux registres

in/out sur les ports I/O

pouvoir spécifier, pour les fonctions sans argument, de ne pas créer une
nouvelle stack frame (mais sans inline pour autant ! call quand même)

C calling convention

inline ASM

```
asm {
  mov %eax, %ebx
}
```

```
naive {
  %eax = %ebx
}
```

On prend la responsabilité de décaler des addresses (qu'un linker usuellement
fait)


## Primitive types

{i,u}{8,16,32,64}
i16, u32, etc

f32 f64

bool

bitarray (unsized type)

utiliser les fonctions MMX pour faire des memcpy

ask not to align a struct


