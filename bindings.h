#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Class Class;

typedef struct Ivar Ivar;

typedef struct Object Object;

typedef struct Property Property;

typedef struct Selector Selector;

typedef struct Vec______Object Vec______Object;

typedef void (*Imp)(const struct Object*_Nonnull , const struct Selector*_Nonnull , struct Vec______Object);

const char *class_getName(Class cls);

Class class_getSuperClass(Class cls);

bool class_isMetaClass(Class cls);

size_t class_getInstanceSize(Class cls);

struct Ivar *class_getInstanceVariable(Class cls, const char *name);

struct Ivar *class_getClassVariable(Class cls, const char *name);

bool class_addIvar(Class cls, const char *name, size_t size, uint8_t alignment, const char *types);

struct Ivar *_Nonnull *class_copyIvarList(Class cls, unsigned int *out_count);

const uint8_t *class_getIvarLayout(Class _cls);

void class_setIvarLayout(Class _cls, const uint8_t *_layout);

const uint8_t *class_weakGetIvarLayout(Class _cls);

void class_weakSetIvarLayout(Class _cls, const uint8_t *_layout);

struct Property *class_getProperty(Class cls, const char *name);

struct Property *_Nonnull *class_copyPropertyList(Class cls, unsigned int *out_count);

bool class_addMethod(Class cls, struct Selector *name, Imp *imp, const char *types);
