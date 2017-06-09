
OBJECTDIR?=./objects

KMOD=hello
SRCS=hello.c
OBJS=${OBJECTDIR}/*.o

rustclean:
	make clean
	cargo clean

.include <bsd.kmod.mk>
