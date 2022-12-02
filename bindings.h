#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_objc_imp Option_objc_imp;

typedef struct Property Property;

typedef struct Selector Selector;

typedef struct objc_ivar objc_ivar;

typedef Repr<ClassData> objc_class;

typedef objc_class *Class;

typedef struct objc_ivar *Ivar;

typedef struct Selector *SEL;

typedef struct Option_objc_imp IMP;

typedef ClassKey Receiver;

typedef Receiver *id;

const char *class_getName(Class cls);

Class class_getSuperClass(Class cls);

bool class_isMetaClass(Class cls);

size_t class_getInstanceSize(Class cls);

Ivar class_getInstanceVariable(Class cls, const char *name);

Ivar class_getClassVariable(Class cls, const char *name);

bool class_addIvar(Class cls, const char *name, size_t size, uint8_t alignment, const char *types);

struct objc_ivar *_Nonnull *class_copyIvarList(Class cls, unsigned int *out_count);

const uint8_t *class_getIvarLayout(Class _cls);

void class_setIvarLayout(Class _cls, const uint8_t *_layout);

const uint8_t *class_weakGetIvarLayout(Class _cls);

void class_weakSetIvarLayout(Class _cls, const uint8_t *_layout);

struct Property *class_getProperty(Class cls, const char *name);

struct Property *_Nonnull *class_copyPropertyList(Class cls, unsigned int *out_count);

bool class_addMethod(Class cls, SEL name, IMP imp, const char *types);

Class objc_allocateClassPair(Class superclass, const char *name, size_t extra_bytes);

void objc_registerClassPair(Class cls);

id class_createInstance(Class cls, size_t _extra_bytes);

void object_getInstanceVariable(id obj, const char *name, void **out_value);

id object_getIvar(id obj, Ivar ivar);

id objc_getMetaClass(const char *name);

struct Selector *sel_registerName(const char *name);
