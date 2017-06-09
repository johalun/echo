#include <sys/param.h>
#include <sys/module.h>
#include <sys/kernel.h>
#include <sys/systm.h>
#include <sys/types.h>
#include <sys/conf.h>
#include <sys/uio.h>
#include <sys/malloc.h>

extern int module_event(struct module *, int, void *);

static moduledata_t module_data = {
    "hello",        /* module name */
     module_event,  /* event handler */
     NULL           /* extra data */
};

DECLARE_MODULE(hello, module_data, SI_SUB_DRIVERS, SI_ORDER_MIDDLE);
