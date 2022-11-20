#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Class Class;

typedef struct Ivar Ivar;

const char *class_getName(Class cls);

Class class_getSuperClass(Class cls);

bool class_isMetaClass(Class cls);

size_t class_getInstanceSize(Class cls);

struct Ivar *class_getInstanceVariable(Class cls, const char *name);

struct Ivar *class_getClassVariable(Class cls, const char *name);

bool class_addIvar(Class cls, const char *name, size_t size, uint8_t alignment, const char *types);

struct Ivar *_Nonnull *class_copyIvarList(Class cls, unsigned int *out_count);
