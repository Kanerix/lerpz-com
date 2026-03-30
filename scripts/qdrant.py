#!/usr/bin/env python3
"""
Simple script to ingest a PDF file into Qdrant.
Usage: python ingest_pdf.py <path_to_pdf>
"""

import sys
import uuid
from pypdf import PdfReader
from qdrant_client import QdrantClient, models

# ─── CONFIG ───────────────────────────────────────────────────────────────────
QDRANT_URL       = "http://localhost:6333"   # or your Qdrant Cloud URL
QDRANT_API_KEY   = None                      # set if using Qdrant Cloud
COLLECTION_NAME  = "pdf_documents"
MODEL_NAME       = "sentence-transformers/all-MiniLM-L6-v2"
VECTOR_SIZE      = 384                       # matches all-MiniLM-L6-v2
CHUNK_SIZE       = 500                       # characters per chunk
CHUNK_OVERLAP    = 50                        # overlap between chunks
# ──────────────────────────────────────────────────────────────────────────────


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


def ensure_collection(client: QdrantClient):
    """Create the Qdrant collection if it doesn't exist."""
    existing = [c.name for c in client.get_collections().collections]
    if COLLECTION_NAME not in existing:
        client.create_collection(
            collection_name=COLLECTION_NAME,
            vectors_config=models.VectorParams(
                size=VECTOR_SIZE,
                distance=models.Distance.COSINE,
            ),
        )
        print(f"✅ Created collection: '{COLLECTION_NAME}'")
    else:
        print(f"✅ Collection '{COLLECTION_NAME}' already exists")


def ingest_pdf(pdf_path: str):
    """Main ingestion pipeline: PDF → chunks → embeddings → Qdrant."""

    # 1. Connect to Qdrant
    client = QdrantClient(
        url=QDRANT_URL,
        api_key=QDRANT_API_KEY,
    )

    # 2. Ensure collection exists
    ensure_collection(client)

    # 3. Extract text from PDF
    print(f"📄 Reading PDF: {pdf_path}")
    text = extract_text_from_pdf(pdf_path)
    if not text.strip():
        print("❌ No text could be extracted from the PDF.")
        sys.exit(1)
    print(f"✅ Extracted {len(text)} characters")

    # 4. Chunk the text
    chunks = chunk_text(text, CHUNK_SIZE, CHUNK_OVERLAP)
    print(f"✅ Split into {len(chunks)} chunks")

    # 5. Build points using FastEmbed (via qdrant-client)
    #    qdrant-client uses FastEmbed under the hood to auto-generate vectors [5]
    points = [
        models.PointStruct(
            id=str(uuid.uuid4()),
            vector=models.Document(text=chunk, model=MODEL_NAME),
            payload={
                "text": chunk,
                "source": pdf_path,
                "chunk_index": i,
            },
        )
        for i, chunk in enumerate(chunks)
    ]

    # 6. Upsert into Qdrant
    print(f"⬆️ Uploading {len(points)} points to Qdrant...")
    client.upsert(
        collection_name=COLLECTION_NAME,
        points=points,
    )
    print(f"🎉 Done! {len(points)} chunks ingested from '{pdf_path}'")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python ingest_pdf.py <path_to_pdf>")
        sys.exit(1)

    pdf_path = sys.argv[1]
    ingest_pdf(pdf_path)
