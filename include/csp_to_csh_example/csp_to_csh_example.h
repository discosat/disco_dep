#ifndef RUST_HEADER
#define RUST_HEADER

extern void rust_char(char c);
extern void rust_wchar(wchar_t c);
extern void rust_short(short i);
extern void rust_ushort(unsigned short i);
extern void rust_int(int i);
extern void rust_uint(unsigned int i);
extern void rust_long(long i);
extern void rust_ulong(unsigned long i);
extern void rust_string(char *s);
extern void rust_csp_conn_print_table();
extern void rust_void(void *s);
extern void rust_int_array(const int *array, int length);
extern void rust_string_array(const char **array, int length);
extern void rust_cstruct(struct CStruct *c_struct);

extern const uint32_t RUST_SLASH_LOAD;
static const uint32_t *const RUST_SLASH_LOAD_LOCK __attribute__((__used__)) = &RUST_SLASH_LOAD;

#endif /* RUST_HEADER */