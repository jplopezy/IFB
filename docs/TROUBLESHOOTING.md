# IFB Troubleshooting

## Linker errors (duplicate main)

- Remove `main.o` from the static library:
  ```bash
  ar d build/lib/libtarget.a main.o
  ```

## Missing symbols

- Verify `IFB_STATIC_LIB_DIR` and `IFB_STATIC_LIBS` are set.
- Ensure all dependencies are linked statically.
