# IFB Troubleshooting

## Build fails: libcurl not found

- Ensure the libcurl dev package is installed.
- `scripts/setup.sh` checks `pkg-config --exists libcurl`.

For a custom static build, set:
- `IFB_STATIC_LIB_DIR`
- `IFB_STATIC_LIBS`
- `IFB_INCLUDE_DIR`

## Linker errors (duplicate main)

If you link a static target library that contains `main`, remove it:

```bash
ar d build/lib/libtarget.a main.o
```

## LLM not activating

- Build with `--features llm`.
- Export either `OPENAI_API_KEY` or `IFB_LLM_URL`.
- Check logs for: `LLM disabled` or `LLM seed injection enabled`.
