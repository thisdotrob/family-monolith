CORS restriction.

Tasks
1. Add Tower-HTTP `CorsLayer` limited to `config::frontend_origin()` with:
   - allowed methods: POST
   - allowed headers: Content-Type, Authorization
   - max age 3600

2. Apply layer before router is served.

3. Integration test: preflight from origin allowed; other origin blocked.

Commit message: "feat(sec): strict CORS layer"
