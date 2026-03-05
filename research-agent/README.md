# Research Agent

Agente de investigación autónomo para análisis de corpus académico interdisciplinario.

## Características

- 🔍 **Búsqueda semántica** en documentos PDF del Hub
- 📊 **Análisis de argumentos** y extracción de claims
- 🔗 **Vinculación conceptual** entre disciplinas
- 📝 **Generación automática** de documentos de investigación
- 📚 **Gestión de citas** con validación
- 🎯 **Integración con Google Workspace**

## Instalación

```bash
cd research-agent
cargo build --release
```

## Uso Rápido

```bash
# Búsqueda en el corpus
research-agent search "game theory legal institutions"

# Iniciar investigación completa
research-agent query "How do legal institutions evolve?"

# Analizar documento específico
research-agent analyze --file "paper.pdf"

# Generar reporte
research-agent synthesize --session <id> --format markdown
```

## Arquitectura

Ver [ARCHITECTURE_RESEARCH_AGENT.md](../ARCHITECTURE_RESEARCH_AGENT.md) para detalles completos.

## Desarrollo

```bash
# Tests
cargo test

# Lint
cargo clippy -- -D warnings

# Run con logs
RUST_LOG=debug cargo run -- search "test query"
```

## Licencia

Apache-2.0
