# Arquitectura: Agente de Investigación Autónomo

## Visión General

Sistema de investigación asistida por IA que combina:
- Motor de búsqueda semántica sobre corpus académico (Hub)
- Análisis y extracción de argumentos
- Síntesis de conocimiento interdisciplinario
- Generación automatizada de documentos de investigación
- Integración con Google Workspace para gestión de outputs

## Stack Tecnológico

### Backend Core
- **Rust** (aprovechando infraestructura gws existente)
  - Parser de PDFs y documentos
  - Sistema de indexación y caché
  - CLI core y orquestación
  
- **Python** (procesamiento NLP y ML)
  - Análisis de texto y extracción de entidades
  - Embeddings semánticos
  - Modelos de argumentación

### Frontend/Interfaz
- **TypeScript + Vite + React** (opcional, para dashboard web)
- **CLI interactivo** (Rust con `clap`)

### Integraciones
- **Google Workspace** (via gws CLI)
- **Hub Files API** (acceso al corpus)
- **LLM APIs** (análisis y síntesis)

## Arquitectura de Componentes

```
┌─────────────────────────────────────────────────────────────┐
│                    RESEARCH AGENT CORE                       │
│                                                               │
│  ┌────────────────────────────────────────────────────┐     │
│  │         1. KNOWLEDGE LAYER (Hub Interface)          │     │
│  │  ├─ Hub Files Reader                                │     │
│  │  ├─ PDF Parser & Text Extractor                    │     │
│  │  ├─ Document Indexer (in-memory + persistent)      │     │
│  │  └─ Semantic Search Engine                         │     │
│  └────────────────────────────────────────────────────┘     │
│                           ↓                                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │       2. ANALYSIS LAYER (NLP & Reasoning)           │     │
│  │  ├─ Argument Extractor                             │     │
│  │  ├─ Claim Identifier                               │     │
│  │  ├─ Evidence Mapper                                │     │
│  │  ├─ Citation Validator                             │     │
│  │  ├─ Contradiction Detector                         │     │
│  │  └─ Concept Linker (cross-disciplinary)           │     │
│  └────────────────────────────────────────────────────┘     │
│                           ↓                                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │      3. SYNTHESIS LAYER (Knowledge Generation)      │     │
│  │  ├─ Research Query Planner                         │     │
│  │  ├─ Multi-source Synthesizer                       │     │
│  │  ├─ Gap Analyzer                                   │     │
│  │  ├─ Outline Generator                              │     │
│  │  └─ Citation Manager                               │     │
│  └────────────────────────────────────────────────────┘     │
│                           ↓                                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │      4. OUTPUT LAYER (Document Generation)          │     │
│  │  ├─ Template Engine                                │     │
│  │  ├─ Google Docs Writer (via gws)                   │     │
│  │  ├─ Markdown Generator                             │     │
│  │  ├─ Bibliography Formatter (multiple styles)       │     │
│  │  └─ Visualization Generator (charts, maps)         │     │
│  └────────────────────────────────────────────────────┘     │
│                           ↓                                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │       5. WORKFLOW ORCHESTRATOR                      │     │
│  │  ├─ State Machine (research pipeline)              │     │
│  │  ├─ Task Queue                                     │     │
│  │  ├─ Memory Manager (context window optimization)   │     │
│  │  └─ Error Recovery & Retry Logic                   │     │
│  └────────────────────────────────────────────────────┘     │
└───────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                   EXTERNAL INTEGRATIONS                      │
│  ├─ Google Workspace (Drive, Docs, Gmail)                   │
│  ├─ Web Search (external knowledge)                         │
│  ├─ Hub Files & Session History                             │
│  └─ LLM APIs (analysis, summarization)                      │
└─────────────────────────────────────────────────────────────┘
```

## Flujo de Trabajo Principal

### Pipeline de Investigación

```
[USER QUERY]
    ↓
[1. QUERY ANALYSIS]
├─ Parse research question
├─ Identify key concepts
├─ Detect required disciplines
└─ Generate search strategy
    ↓
[2. KNOWLEDGE RETRIEVAL]
├─ Search Hub corpus
├─ Rank documents by relevance
├─ Extract relevant passages
└─ Build evidence base
    ↓
[3. DEEP ANALYSIS]
├─ Extract arguments from sources
├─ Identify claims and premises
├─ Map relationships between concepts
├─ Detect contradictions/agreements
└─ Find knowledge gaps
    ↓
[4. EXTERNAL AUGMENTATION] (optional)
├─ Identify gaps in Hub knowledge
├─ Web search for additional sources
├─ Evaluate new sources
└─ Integrate into evidence base
    ↓
[5. SYNTHESIS]
├─ Generate document outline
├─ Organize arguments logically
├─ Create narrative structure
├─ Prepare citations
└─ Generate visualizations
    ↓
[6. DOCUMENT GENERATION]
├─ Write sections with evidence
├─ Format citations
├─ Create Google Doc
├─ Add visualizations
└─ Generate bibliography
    ↓
[7. DELIVERY]
├─ Save to Google Drive
├─ Send notification email
├─ Generate summary report
└─ Store in session history
```

## Estructura de Directorios

```
/home/user/webapp/
├─ research-agent/           # Nuevo módulo
│  ├─ src/
│  │  ├─ core/              # Lógica central
│  │  │  ├─ mod.rs
│  │  │  ├─ agent.rs        # Orquestador principal
│  │  │  ├─ pipeline.rs     # Pipeline de investigación
│  │  │  └─ state.rs        # Gestión de estado
│  │  │
│  │  ├─ knowledge/         # Capa de conocimiento
│  │  │  ├─ mod.rs
│  │  │  ├─ hub_reader.rs   # Interface con Hub
│  │  │  ├─ pdf_parser.rs   # Extracción de texto
│  │  │  ├─ indexer.rs      # Indexación de documentos
│  │  │  └─ search.rs       # Motor de búsqueda
│  │  │
│  │  ├─ analysis/          # Capa de análisis
│  │  │  ├─ mod.rs
│  │  │  ├─ argument_extractor.rs
│  │  │  ├─ claim_detector.rs
│  │  │  ├─ citation_validator.rs
│  │  │  └─ concept_linker.rs
│  │  │
│  │  ├─ synthesis/         # Capa de síntesis
│  │  │  ├─ mod.rs
│  │  │  ├─ planner.rs
│  │  │  ├─ synthesizer.rs
│  │  │  ├─ gap_analyzer.rs
│  │  │  └─ outline_generator.rs
│  │  │
│  │  ├─ output/            # Capa de salida
│  │  │  ├─ mod.rs
│  │  │  ├─ template.rs
│  │  │  ├─ gdocs_writer.rs
│  │  │  ├─ markdown.rs
│  │  │  └─ bibliography.rs
│  │  │
│  │  ├─ utils/             # Utilidades
│  │  │  ├─ mod.rs
│  │  │  ├─ cache.rs
│  │  │  ├─ text_processing.rs
│  │  │  └─ llm_client.rs
│  │  │
│  │  └─ lib.rs
│  │
│  ├─ python/               # Componentes Python
│  │  ├─ nlp/
│  │  │  ├─ embeddings.py
│  │  │  ├─ entity_extraction.py
│  │  │  └─ similarity.py
│  │  │
│  │  ├─ models/
│  │  │  └─ argument_mining.py
│  │  │
│  │  └─ requirements.txt
│  │
│  ├─ templates/            # Templates de documentos
│  │  ├─ research_paper.md
│  │  ├─ literature_review.md
│  │  ├─ theoretical_analysis.md
│  │  └─ synthesis_report.md
│  │
│  ├─ config/               # Configuraciones
│  │  ├─ default.toml
│  │  └─ disciplines.toml
│  │
│  ├─ Cargo.toml
│  └─ README.md
│
├─ skills/
│  └─ gws-research-agent/   # Nueva skill
│     └─ SKILL.md
│
└─ (archivos existentes de gws)
```

## Módulos Detallados

### 1. Knowledge Layer

**Hub Reader**
- Interface con `hub_files_tool`
- Caché local de documentos leídos
- Gestión de límites de rate

**PDF Parser**
```rust
pub struct Document {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: Option<String>,
    pub sections: Vec<Section>,
    pub references: Vec<Reference>,
    pub metadata: Metadata,
}

pub struct Section {
    pub heading: String,
    pub level: u8,
    pub content: String,
    pub page_range: (u32, u32),
}
```

**Indexer**
- Índice invertido para búsqueda rápida
- Embeddings semánticos (opcional)
- Caché persistente (SQLite)

**Search Engine**
- BM25 ranking
- Semantic similarity (embeddings)
- Hybrid scoring
- Filter by discipline/author/year

### 2. Analysis Layer

**Argument Extractor**
```rust
pub struct Argument {
    pub claim: String,
    pub premises: Vec<String>,
    pub evidence: Vec<Evidence>,
    pub source: SourceLocation,
    pub confidence: f32,
}

pub struct Evidence {
    pub text: String,
    pub evidence_type: EvidenceType, // Empirical, Logical, Citation
    pub strength: f32,
}
```

**Claim Detector**
- Identificar proposiciones centrales
- Distinguir claims vs. background
- Clasificar tipo de claim (normativo, descriptivo, causal)

**Citation Validator**
- Verificar que citas sean precisas
- Detectar "citation needed"
- Sugerir fuentes del corpus

**Concept Linker**
- Mapear conceptos entre disciplinas
- Detectar analogías
- Construir grafo de conocimiento

### 3. Synthesis Layer

**Research Query Planner**
```rust
pub struct ResearchQuery {
    pub main_question: String,
    pub sub_questions: Vec<String>,
    pub disciplines: Vec<String>,
    pub search_strategy: SearchStrategy,
    pub output_format: OutputFormat,
}

pub struct SearchStrategy {
    pub keywords: Vec<String>,
    pub must_include_authors: Vec<String>,
    pub date_range: Option<(u16, u16)>,
    pub document_types: Vec<DocType>,
}
```

**Multi-source Synthesizer**
- Combinar evidencia de múltiples fuentes
- Resolver contradicciones
- Ponderar calidad de fuentes

**Gap Analyzer**
- Identificar qué falta en el corpus
- Sugerir búsquedas externas
- Detectar sesgos en la literatura

**Outline Generator**
- Estructura lógica del documento
- Secuenciación de argumentos
- Balance entre secciones

### 4. Output Layer

**Template Engine**
- Soporte para múltiples formatos de output
- Variables dinámicas
- Conditional sections

**Google Docs Writer**
```rust
pub struct GDocsWriter {
    gws_client: GwsClient,
}

impl GDocsWriter {
    pub async fn create_document(&self, content: &Document) -> Result<String> {
        // 1. Create doc via gws
        // 2. Insert formatted content
        // 3. Add citations as footnotes
        // 4. Insert visualizations
        // 5. Apply formatting
    }
}
```

**Bibliography Formatter**
- APA, MLA, Chicago, etc.
- Auto-format from structured data
- Alphabetization

### 5. Workflow Orchestrator

**State Machine**
```rust
pub enum ResearchState {
    Initialized,
    AnalyzingQuery,
    SearchingCorpus(SearchProgress),
    AnalyzingEvidence,
    ExternalSearchRequired(Vec<String>),
    Synthesizing,
    GeneratingDocument,
    Complete(OutputArtifacts),
    Error(ErrorDetails),
}

pub struct ResearchSession {
    pub id: Uuid,
    pub state: ResearchState,
    pub query: ResearchQuery,
    pub evidence_base: Vec<Evidence>,
    pub artifacts: OutputArtifacts,
    pub metadata: SessionMetadata,
}
```

## Configuración

### config/default.toml
```toml
[search]
max_results = 50
similarity_threshold = 0.7
use_semantic_search = true

[analysis]
min_argument_confidence = 0.6
extract_citations = true
validate_claims = true

[synthesis]
max_sources_per_claim = 5
require_multi_source_validation = true

[output]
default_format = "google_docs"
include_bibliography = true
citation_style = "APA"

[llm]
provider = "openai"  # or "anthropic", "google"
model = "gpt-4"
temperature = 0.3
max_tokens = 4000
```

### config/disciplines.toml
```toml
[disciplines.game_theory]
keywords = ["nash equilibrium", "prisoner's dilemma", "coordination game"]
core_authors = ["Nash", "von Neumann", "Aumann", "Maynard Smith"]

[disciplines.legal_theory]
keywords = ["extended phenotype", "institutional design", "memetic fitness"]
core_authors = ["Lerer", "North", "Hayek"]

[disciplines.philosophy]
keywords = ["consciousness", "free will", "intentional stance"]
core_authors = ["Dennett", "Bunge"]
```

## API Interface

### CLI Commands

```bash
# Iniciar nueva investigación
research-agent query "¿Cómo se aplica la teoría de juegos al diseño institucional?"

# Búsqueda en corpus
research-agent search --query "nash equilibrium legal systems" --max-results 10

# Analizar documento específico
research-agent analyze --file "LAW AS EXTENDED PHENOTYPE.pdf" --extract-arguments

# Generar reporte
research-agent synthesize --session <id> --format google-docs --output "Research Report"

# Explorar corpus
research-agent corpus stats
research-agent corpus map --discipline "game_theory"
```

### Programmatic API (Rust)

```rust
use research_agent::ResearchAgent;

#[tokio::main]
async fn main() -> Result<()> {
    let agent = ResearchAgent::new().await?;
    
    let query = ResearchQuery::builder()
        .question("¿Cómo evolucionan las instituciones legales?")
        .disciplines(vec!["legal_theory", "evolutionary_biology"])
        .output_format(OutputFormat::GoogleDocs)
        .build();
    
    let session = agent.start_research(query).await?;
    
    // Monitor progress
    while !session.is_complete() {
        let state = session.get_state().await?;
        println!("State: {:?}", state);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    
    let artifacts = session.get_artifacts().await?;
    println!("Document created: {}", artifacts.gdocs_url);
    
    Ok(())
}
```

## Fases de Implementación

### Fase 1: Fundaciones (Semanas 1-2)
- ✅ Hub Files Reader
- ✅ PDF Parser básico
- ✅ Indexer simple (in-memory)
- ✅ CLI básico

### Fase 2: Búsqueda y Análisis (Semanas 3-4)
- 🔨 Search engine (BM25 + semantic)
- 🔨 Argument extractor
- 🔨 Citation validator
- 🔨 Caché persistente

### Fase 3: Síntesis (Semanas 5-6)
- 🔨 Query planner
- 🔨 Multi-source synthesizer
- 🔨 Outline generator
- 🔨 Template engine

### Fase 4: Generación de Output (Semanas 7-8)
- 🔨 Google Docs writer
- 🔨 Bibliography formatter
- 🔨 Markdown generator
- 🔨 Visualizations

### Fase 5: Orquestación (Semanas 9-10)
- 🔨 State machine completo
- 🔨 Pipeline end-to-end
- 🔨 Error handling robusto
- 🔨 Testing integral

### Fase 6: Refinamiento (Semanas 11-12)
- 🎨 Dashboard web (opcional)
- 📊 Visualizaciones avanzadas
- 🚀 Optimizaciones de performance
- 📚 Documentación completa

## Casos de Uso Prioritarios

### 1. Literature Review Automation
**Input:** "Review game theory applications in legal systems"
**Output:** Google Doc con:
- Resumen de estado del arte
- Autores principales y sus contribuciones
- Evolución histórica del campo
- Gaps de investigación
- Bibliografía completa

### 2. Cross-Disciplinary Synthesis
**Input:** "Connect Dennett's concept of 'memes' with Hayek's spontaneous order"
**Output:** Análisis que:
- Extrae definiciones de ambos autores
- Identifica puntos de contacto
- Resalta diferencias y complementariedades
- Sugiere implicaciones para diseño institucional

### 3. Argument Validation
**Input:** "Validate the claim: 'Legal systems evolve through memetic selection'"
**Output:** Reporte con:
- Evidencia a favor en el corpus
- Evidencia en contra
- Calidad de cada fuente
- Conclusión balanceada

### 4. Research Gap Detection
**Input:** "What's missing in my corpus on evolutionary psychology?"
**Output:**
- Temas cubiertos
- Temas no cubiertos (comparado con field estándar)
- Autores faltantes
- Papers sugeridos para agregar

## Métricas de Éxito

- **Precisión de búsqueda:** >80% de resultados relevantes en top 10
- **Calidad de citas:** >95% de citas correctamente atribuidas
- **Cobertura de corpus:** 100% de documentos indexables accesibles
- **Tiempo de generación:** <5 min para reporte de 2000 palabras
- **Satisfacción de usuario:** Documentos generados requieren <20% de edición manual

## Próximos Pasos Inmediatos

1. **Crear branch de desarrollo**
2. **Implementar Hub Reader básico**
3. **Parsear primer PDF de prueba**
4. **Construir índice mínimo viable**
5. **CLI de búsqueda simple**

---

**¿Comenzamos con la Fase 1?**
