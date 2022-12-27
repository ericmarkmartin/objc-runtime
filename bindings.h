#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Option_objc_imp Option_objc_imp;

typedef struct Property Property;

typedef struct objc_ivar objc_ivar;

typedef struct objc_selector objc_selector;

typedef Repr<ClassData> objc_class;

typedef objc_class *Class;

typedef struct objc_ivar *Ivar;

typedef struct objc_selector *SEL;

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

id class_createInstance(Class cls, size_t _extra_bytes);

Class objc_allocateClassPair(Class superclass, const char *name, size_t extra_bytes);

id objc_getClass(const char *name);

void objc_registerClassPair(Class cls);

id objc_getMetaClass(const char *name);

id object_getIvar(id obj, Ivar ivar);

Class object_getClass(id obj);

ptrdiff_t ivar_getOffset(Ivar ivar);

void object_setIvar(id obj, Ivar ivar, id value);

Ivar object_getInstanceVariable(id obj, const char *name, void **out_value);

Ivar object_setInstanceVariable(id obj, const char *name, void *value);

SEL sel_registerName(const char *name);

const char *sel_getName(SEL sel);
