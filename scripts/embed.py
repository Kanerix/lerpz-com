#!/usr/bin/env python3
"""
Ingest a PDF file into Qdrant using Portkey/OpenAI embeddings.

Usage: python embed.py <path_to_config.env> <path_to_pdf>

Required .env keys:
    PORTKEY_BASE_URL          - Portkey gateway URL
    PORTKEY_PROVIDER          - Provider name, e.g. "openai"
    PORTKEY_API_KEY           - Portkey API key
    DEFAULT_EMBEDDING_MODEL   - Embedding model name, e.g. "text-embedding-3-small"
    QDRANT_URL                - Qdrant instance URL
    QDRANT_COLLECTION         - Qdrant collection name

Optional .env keys:
    CHUNK_SIZE                - Characters per chunk (default: 500)
    CHUNK_OVERLAP             - Overlap between chunks (default: 50)
    VECTOR_SIZE               - Embedding dimension (default: 1536)
"""

import sys
import uuid
from pathlib import Path

from dotenv import dotenv_values
from openai import OpenAI
from portkey_ai import createHeaders
from pypdf import PdfReader
from qdrant_client import QdrantClient, models

DEFAULT_VECTOR_SIZE = 1536
DEFAULT_CHUNK_SIZE = 500
DEFAULT_CHUNK_OVERLAP = 50

REQUIRED_KEYS = [
    "PORTKEY_BASE_URL",
    "PORTKEY_PROVIDER",
    "PORTKEY_API_KEY",
    "DEFAULT_EMBEDDING_MODEL",
    "QDRANT_URL",
    "QDRANT_COLLECTION",
]


def load_config(env_path: str) -> dict:
    """Load and validate configuration from a .env file."""
    path = Path(env_path)
    if not path.exists():
        print(f"❌ Config file not found: {env_path}")
        sys.exit(1)

    config = dotenv_values(path)

    missing = [k for k in REQUIRED_KEYS if not config.get(k)]
    if missing:
        print(f"❌ Missing required config keys: {', '.join(missing)}")
        sys.exit(1)

    return config


def build_openai_client(config: dict) -> OpenAI:
    """Build an OpenAI client routed through the Portkey gateway."""
    return OpenAI(
        api_key=config["PORTKEY_API_KEY"],
        base_url=config["PORTKEY_BASE_URL"],
        default_headers=createHeaders(
            api_key=config["PORTKEY_API_KEY"],
            provider=config["PORTKEY_PROVIDER"],
        ),
    )


def extract_text_from_pdf(pdf_path: str) -> str:
    """Extract all text from a PDF file."""
    reader = PdfReader(pdf_path)
    text = ""
    for page in reader.pages:
        page_text = page.extract_text()
        if page_text:
            text += page_text + "\n"
    return text


def chunk_text(text: str, chunk_size: int, overlap: int) -> list[str]:
    """Split text into overlapping chunks."""
    chunks = []
    start = 0
    while start < len(text):
        end = start + chunk_size
        chunk = text[start:end].strip()
        if chunk:
            chunks.append(chunk)
        start += chunk_size - overlap
    return chunks


def embed_chunks(client: OpenAI, chunks: list[str], model: str) -> list[list[float]]:
    """Request embeddings for all chunks via Portkey → provider."""
    print(f"🔢 Embedding {len(chunks)} chunks with model '{model}'...")
    response = client.embeddings.create(
        input=chunks,
        model=model,
    )
    vectors = [item.embedding for item in response.data]
    print(f"✅ Received {len(vectors)} embeddings (dim={len(vectors[0])})")
    return vectors


def ensure_collection(client: QdrantClient, collection_name: str, vector_size: int):
    """Create the Qdrant collection if it doesn't already exist."""
    existing = [c.name for c in client.get_collections().collections]
    if collection_name not in existing:
        client.create_collection(
            collection_name=collection_name,
            vectors_config=models.VectorParams(
                size=vector_size,
                distance=models.Distance.COSINE,
            ),
        )
        print(f"✅ Created collection: '{collection_name}'")
    else:
        print(f"✅ Collection '{collection_name}' already exists")


def ingest_pdf(config: dict, pdf_path: str):
    """Main ingestion pipeline: PDF → chunks → embeddings → Qdrant."""

    collection_name = config["QDRANT_COLLECTION"]
    embedding_model = config["DEFAULT_EMBEDDING_MODEL"]
    vector_size = int(config.get("VECTOR_SIZE", DEFAULT_VECTOR_SIZE))
    chunk_size = int(config.get("CHUNK_SIZE", DEFAULT_CHUNK_SIZE))
    chunk_overlap = int(config.get("CHUNK_OVERLAP", DEFAULT_CHUNK_OVERLAP))

    print(config["QDRANT_URL"])

    # 1. Connect to Qdrant
    qdrant = QdrantClient(
        url=config["QDRANT_URL"],
        api_key=config.get("QDRANT_API_KEY"),
    )

    # 2. Build Portkey → OpenAI client
    openai = build_openai_client(config)

    # 3. Ensure collection exists
    ensure_collection(qdrant, collection_name, vector_size)

    # 4. Extract text from PDF
    print(f"📄 Reading PDF: {pdf_path}")
    text = extract_text_from_pdf(pdf_path)
    if not text.strip():
        print("❌ No text could be extracted from the PDF.")
        sys.exit(1)
    print(f"✅ Extracted {len(text)} characters")

    # 5. Chunk the text
    chunks = chunk_text(text, chunk_size, chunk_overlap)
    print(f"✅ Split into {len(chunks)} chunks")

    # 6. Embed all chunks via Portkey → provider
    vectors = embed_chunks(openai, chunks, embedding_model)

    # 7. Build Qdrant points
    points = [
        models.PointStruct(
            id=str(uuid.uuid4()),
            vector=vector,
            payload={
                "text": chunk,
                "source": pdf_path,
                "chunk_index": i,
            },
        )
        for i, (chunk, vector) in enumerate(zip(chunks, vectors))
    ]

    # 8. Upsert into Qdrant
    print(f"⬆️  Uploading {len(points)} points to '{collection_name}'...")
    qdrant.upsert(
        collection_name=collection_name,
        points=points,
    )
    print(f"🎉 Done! {len(points)} chunks from '{pdf_path}' → '{collection_name}'")


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python embed.py <path_to_config.env> <path_to_pdf>")
        sys.exit(1)

    env_path = sys.argv[1]
    pdf_path = sys.argv[2]

    if not Path(pdf_path).exists():
        print(f"❌ PDF not found: {pdf_path}")
        sys.exit(1)

    config = load_config(env_path)
    ingest_pdf(config, pdf_path)
