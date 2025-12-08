# RAG Enhancement Solutions

This document records all RAG (Retrieval Augmented Generation) solutions researched and implemented for the Local AI Assistant project.

## Current Implementation

### Tech Stack
- **Embedding Model**: Kalosm BERT (768D vectors)
- **Vector Store**: SurrealDB with EmbeddingIndexedTable
- **LLM**: Qwen 2.5 7B (via Kalosm)
- **Chunking**: SemanticChunker (default configuration)

### Current Configuration (v0.1.0-rag-optimization)

| Parameter | Value | Location |
|-----------|-------|----------|
| Search Results Count | 10 | `vector_store.rs:31` |
| Similarity Threshold | 0.5 | `vector_store.rs:33` |
| Max Results | 5 | `vector_store.rs:35` |

---

## Solution Options

### Option A: Quick Optimization (Implemented)

**Complexity**: Low | **Impact**: Medium-High

#### Changes Made
1. **Threshold Filtering** (`vector_store.rs`)
   - Search 10 results initially
   - Filter by similarity threshold (0.5)
   - Return max 5 results

2. **Relevance Scores** (`server_functions/chat.rs`)
   - Format: `[Reference N] (Relevance: XX%)`
   - Log document count for debugging

3. **Enhanced RAG Prompt** (`components/chat.rs`)
   - Clear `=== REFERENCE DOCUMENTS ===` delimiters
   - Numbered instructions for LLM
   - Explicit citation request
   - Fallback instruction for missing context

---

### Option B: Hybrid Search (Future)

**Complexity**: Medium | **Impact**: High

Combines semantic search with keyword search for better recall.

```rust
pub async fn hybrid_search(query: &str) -> Vec<Document> {
    let semantic = vector_search(query, 0.7).await;  // 70% weight
    let keyword = keyword_search(query, 0.3).await;  // 30% weight
    merge_and_dedupe(semantic, keyword)
}
```

**Implementation Steps**:
1. Add keyword search using SurrealDB `CONTAINS` or full-text search
2. Implement RRF (Reciprocal Rank Fusion) for result merging
3. Update `search_context()` to use hybrid search

---

### Option C: LLM Reranking (Future)

**Complexity**: Medium-High | **Impact**: High

Use the LLM itself to rerank search results by relevance.

```rust
pub async fn rerank(query: &str, docs: Vec<Document>) -> Vec<Document> {
    for doc in &mut docs {
        let prompt = format!(
            "Rate relevance 0-10:\nQuery: {}\nDoc: {}\nScore:",
            query, doc.body
        );
        doc.score = llm_score(prompt).await;
    }
    docs.sort_by(|a, b| b.score.cmp(&a.score));
    docs
}
```

**Considerations**:
- Adds latency (LLM call per document)
- Can batch multiple documents in one call
- Significantly improves result quality

---

### Option D: Upgrade Embedding Model (Future)

**Complexity**: Medium | **Impact**: Medium-High

Replace Kalosm BERT with a better embedding model.

**Options**:
| Model | Dimensions | Pros | Cons |
|-------|------------|------|------|
| BERT-large | 1024D | Better quality | Larger size |
| Sentence-Transformers | 768D | Optimized for semantic search | Requires Candle |
| E5-large | 1024D | State-of-the-art retrieval | More compute |

**Implementation**:
```rust
use candle_transformers::models::bert::BertModel;
// Load custom model from HuggingFace
```

---

### Option E: Full Candle RAG Stack (Long-term)

**Complexity**: High | **Impact**: Very High

Complete migration to Candle ecosystem.

**Components**:
- Replace Kalosm with Candle for LLM inference
- Replace SurrealDB with LanceDB (Rust-native vector DB)
- Full HuggingFace ecosystem integration

**Risks**:
- Large refactoring effort
- Breaking changes
- Learning curve

---

## Recommended Implementation Path

```
Phase 1 (Done): Option A - Quick Optimization
     ↓
Phase 2 (Next): Option B - Hybrid Search
     ↓
Phase 3 (Later): Option C or D - Reranking/Better Embeddings
     ↓
Phase 4 (Future): Option E - Full Candle (Optional)
```

---

## Industry References

| Framework | RAG Features | Link |
|-----------|-------------|------|
| **Candle** | HuggingFace official Rust ML, BERT/LLaMA support | [GitHub](https://github.com/huggingface/candle) |
| **Kalosm** | Current stack, simple but limited | [Docs](https://docs.rs/kalosm) |
| **LanceDB** | Rust-native vector DB, Candle integration | [Article](https://medium.com/data-science/scale-up-your-rag-a-rust-powered-indexing-pipeline-with-lancedb-and-candle-cc681c6162e8) |
| **Orca** | Candle-based RAG CLI framework | [Blog](https://huggingface.co/blog/santiagomed/building-a-rag-cli-application-application) |

---

## Configuration Tuning Guide

### Similarity Threshold

| Threshold | Use Case |
|-----------|----------|
| 0.3 - 0.4 | High recall, more results, lower precision |
| 0.5 - 0.6 | Balanced (current setting) |
| 0.7 - 0.8 | High precision, fewer but better results |

### Search Results Count

- **Too few (< 5)**: May miss relevant documents
- **5-10**: Good balance for filtering
- **Too many (> 20)**: Slower processing, diminishing returns

### Prompt Engineering Tips

1. Use clear section delimiters (`===`)
2. Number references for easy citation
3. Include explicit instructions for LLM behavior
4. Provide fallback instruction for missing context
5. Keep context concise but complete

---

## Changelog

### v0.1.0-rag-optimization (2024-12)
- Added similarity threshold filtering (0.5)
- Increased search results from 3 to 10
- Added relevance scores to context format
- Enhanced RAG prompt with citation instructions
