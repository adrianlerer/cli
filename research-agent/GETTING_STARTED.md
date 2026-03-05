# Getting Started with Research Agent

## 🎯 What is Research Agent?

Research Agent is an autonomous system for academic research that:
- Searches your Hub corpus of 54 academic documents
- Extracts arguments and analyzes claims
- Synthesizes knowledge across disciplines
- Generates research documents automatically
- Integrates with Google Workspace

## 🚀 Quick Start

### Option 1: TypeScript/Node.js (Fastest)

```bash
cd typescript
npm install
npm run dev init
npm run dev search "game theory"
```

### Option 2: Python Prototype

```bash
cd python
pip install -r requirements.txt
python hub_search.py "nash equilibrium"
```

### Option 3: Rust (Full Implementation)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Run
./target/release/research-agent --help
```

## 📚 Your Corpus

Your Hub contains 54 interdisciplinary documents:

**Game Theory & Economics**
- von Neumann & Morgenstern: Theory of Games
- Nash: Non-Cooperative Games
- Aumann: Common Knowledge
- Maynard Smith: Evolutionary Game Theory

**Legal Theory**
- Extended Phenotype applications to law
- Game Theory and Legal Systems
- Institutional evolution

**Philosophy**
- Daniel Dennett (5 books): Consciousness, Free Will, Memes
- Mario Bunge: Philosophy of Science

**Psychology & Social Science**
- Coalitional Psychology
- Group Dynamics
- Kahneman: Noise and Bias

**Political Philosophy**
- James C. Scott: Resistance and State Power
- Institutional Design

## 🔍 Basic Commands

### Search Corpus
```bash
research-agent search "legal institutions evolution"
```

### List All Files
```bash
research-agent corpus --list
```

### Show Statistics
```bash
research-agent corpus --stats
```

### Analyze Document
```bash
research-agent analyze "LAW AS EXTENDED PHENOTYPE.pdf" --extract-arguments
```

### Start Research Session
```bash
research-agent query "How do legal institutions evolve?" --format markdown
```

## 🎓 Example Workflows

### 1. Literature Review

```bash
# Search for all game theory papers
research-agent search "game theory" --max-results 20

# Analyze key arguments
research-agent analyze "Theory of Games.pdf" --extract-arguments

# Generate literature review
research-agent synthesize --session <id> --format google-docs
```

### 2. Cross-Disciplinary Synthesis

```bash
# Query spanning disciplines
research-agent query "Connect Dennett's memes with Hayek's spontaneous order"

# System will:
# 1. Search both philosophy and economics papers
# 2. Extract relevant concepts
# 3. Identify connections
# 4. Generate synthesis document
```

### 3. Argument Validation

```bash
# Check a specific claim
research-agent validate "Legal systems evolve through memetic selection"

# System will:
# 1. Find supporting evidence in corpus
# 2. Find contradicting evidence
# 3. Assess quality of sources
# 4. Provide balanced conclusion
```

## 🛠️ Configuration

Edit `config/default.toml`:

```toml
[search]
max_results = 50
similarity_threshold = 0.7

[output]
default_format = "markdown"  # or "google_docs"
citation_style = "APA"       # or "MLA", "Chicago"
```

## 🧪 Development

### Run Tests
```bash
cargo test
npm test
python -m pytest
```

### Enable Debug Logging
```bash
export RUST_LOG=debug
research-agent search "test"
```

### Run Demo
```bash
./demo.sh
```

## 📖 Learn More

- [Architecture](../ARCHITECTURE_RESEARCH_AGENT.md) - Full system design
- [README](README.md) - Technical details
- [API Documentation](docs/api.md) - Programmatic usage

## 🎯 Next Steps

1. **Try the demo**: `./demo.sh`
2. **Search your corpus**: `research-agent search "your topic"`
3. **Explore capabilities**: Read the architecture doc
4. **Customize**: Edit config files
5. **Extend**: Add your own analysis modules

## 🤝 Contributing

This is part of the larger `gws` (Google Workspace CLI) project. Contributions welcome!

## 📝 License

Apache-2.0
