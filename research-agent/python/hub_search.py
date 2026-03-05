#!/usr/bin/env python3
"""
Hub Search - Python wrapper for searching the research corpus
Can be used while the Rust implementation is being built
"""

import sys
import subprocess
import json
from typing import List, Dict, Optional
from dataclasses import dataclass


@dataclass
class HubFile:
    """Represents a file in the Hub"""
    file_name: str
    summary: Optional[str] = None


@dataclass
class SearchResult:
    """Represents a search result"""
    file_name: str
    score: float
    summary: Optional[str]
    matched_lines: List[str]


class HubSearcher:
    """Search interface for Hub corpus"""
    
    def __init__(self):
        pass
    
    def list_files(self) -> List[HubFile]:
        """List all files in the Hub"""
        # This would call hub_files_tool via subprocess
        # For now, return example structure
        print("📚 Listing files from Hub...")
        print("⚠️  Full implementation requires hub_files_tool CLI")
        return []
    
    def grep_files(self, pattern: str, file_pattern: str = "*.pdf") -> Dict[str, List[str]]:
        """Grep files in the Hub"""
        print(f"🔍 Searching for: {pattern}")
        print("⚠️  Full implementation requires hub_files_tool CLI")
        return {}
    
    def search(self, query: str, max_results: int = 10) -> List[SearchResult]:
        """Search the Hub corpus"""
        print(f"🔎 Searching corpus for: '{query}'")
        print(f"   Max results: {max_results}")
        
        # Grep for the query
        matches = self.grep_files(query)
        
        # Calculate scores and build results
        results = []
        for file_name, lines in matches.items():
            score = self._calculate_score(lines, query)
            results.append(SearchResult(
                file_name=file_name,
                score=score,
                summary=None,  # Would fetch from Hub
                matched_lines=lines
            ))
        
        # Sort by score
        results.sort(key=lambda x: x.score, reverse=True)
        
        return results[:max_results]
    
    def _calculate_score(self, lines: List[str], query: str) -> float:
        """Simple scoring function"""
        num_matches = len(lines)
        query_words = query.lower().split()
        
        word_hits = 0
        for line in lines:
            line_lower = line.lower()
            for word in query_words:
                if word in line_lower:
                    word_hits += 1
        
        word_coverage = word_hits / max(len(query_words), 1)
        
        import math
        return math.log10(num_matches + 1) + word_coverage


def main():
    """CLI interface"""
    if len(sys.argv) < 2:
        print("Usage: python hub_search.py <query>")
        print("Example: python hub_search.py 'game theory'")
        sys.exit(1)
    
    query = " ".join(sys.argv[1:])
    
    searcher = HubSearcher()
    results = searcher.search(query)
    
    if not results:
        print("\n❌ No results found")
        return
    
    print(f"\n✅ Found {len(results)} results:\n")
    
    for i, result in enumerate(results, 1):
        print(f"{i}. {result.file_name}")
        print(f"   Score: {result.score:.3f}")
        if result.summary:
            print(f"   {result.summary}")
        print()


if __name__ == "__main__":
    main()
