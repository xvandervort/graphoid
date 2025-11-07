#!/usr/bin/env python3
"""
Glang Self-Hosting Metrics Tracker

Measures Glang's progress toward self-hosting by analyzing:
1. Lines of code ratio (Python vs Glang)
2. Module independence (pure Glang vs hybrid vs wrapper)
3. Functionality coverage by category
"""

import os
import re
import json
from pathlib import Path
from typing import Dict, List, Tuple
from collections import defaultdict

class SelfHostingMetrics:
    def __init__(self, repo_root: str = "."):
        self.repo_root = Path(repo_root)
        self.python_src = self.repo_root / "src" / "glang"
        self.stdlib = self.repo_root / "stdlib"
        self.samples = self.repo_root / "samples"

        # Module classification patterns
        self.builtin_patterns = [
            r'_builtin_\w+',  # Direct builtin calls
            r'import\s+"[^"]+"\s+as\s+\w+',  # Python module imports
            r'http\.',  # Network operations
            r'html_parser\.',  # HTML parsing
            r'io\.open',  # File I/O
        ]

    def count_lines(self, path: Path, extension: str) -> int:
        """Count non-empty, non-comment lines in files with given extension."""
        total = 0
        for file_path in path.rglob(f"*.{extension}"):
            if "__pycache__" in str(file_path):
                continue
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    lines = f.readlines()
                    for line in lines:
                        stripped = line.strip()
                        if stripped and not stripped.startswith('#'):
                            total += 1
            except:
                pass
        return total

    def classify_module(self, file_path: Path) -> str:
        """Classify a .gr module as pure/hybrid/wrapper."""
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Count builtin/Python dependencies
        builtin_count = 0
        for pattern in self.builtin_patterns:
            builtin_count += len(re.findall(pattern, content))

        # Count total lines (non-empty, non-comment)
        lines = [l.strip() for l in content.split('\n')]
        code_lines = [l for l in lines if l and not l.startswith('#')]
        total_lines = len(code_lines)

        if total_lines == 0:
            return "empty"

        # Classification heuristics
        builtin_density = builtin_count / total_lines

        if builtin_density < 0.01:  # Less than 1% builtin calls
            return "pure"
        elif builtin_density < 0.1:  # Less than 10% builtin calls
            return "hybrid"
        else:
            return "wrapper"

    def analyze_modules(self) -> Dict:
        """Analyze all stdlib modules."""
        modules = {}
        for gr_file in self.stdlib.glob("*.gr"):
            classification = self.classify_module(gr_file)
            modules[gr_file.stem] = {
                'classification': classification,
                'lines': self.count_lines_in_file(gr_file)
            }
        return modules

    def count_lines_in_file(self, path: Path) -> int:
        """Count non-empty, non-comment lines in a single file."""
        try:
            with open(path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
                count = 0
                for line in lines:
                    stripped = line.strip()
                    if stripped and not stripped.startswith('#'):
                        count += 1
                return count
        except:
            return 0

    def calculate_functionality_coverage(self) -> Dict[str, float]:
        """Estimate functionality coverage by category."""
        # These are estimates based on analysis
        return {
            'data_structures': 95,  # Lists, maps, trees mostly in Glang
            'string_processing': 85,  # Enhanced string methods
            'mathematical': 90,  # Conversions, statistics
            'io_operations': 15,  # File handles, network still Python
            'language_core': 5,  # Parser, execution in Python
            'network': 30,  # HTTP operations partially Glang
            'parsing': 40,  # JSON/HTML parsing mixed
        }

    def generate_report(self) -> str:
        """Generate comprehensive metrics report."""
        # Count lines of code
        python_lines = self.count_lines(self.python_src, "py")
        glang_stdlib_lines = self.count_lines(self.stdlib, "gr")
        glang_samples_lines = self.count_lines(self.samples, "gr")

        # Analyze modules
        modules = self.analyze_modules()
        pure_modules = [m for m, d in modules.items() if d['classification'] == 'pure']
        hybrid_modules = [m for m, d in modules.items() if d['classification'] == 'hybrid']
        wrapper_modules = [m for m, d in modules.items() if d['classification'] == 'wrapper']

        # Calculate ratios
        lcr = glang_stdlib_lines / (python_lines + glang_stdlib_lines) * 100
        mir = len(pure_modules) / len(modules) * 100 if modules else 0

        # Get functionality coverage
        func_coverage = self.calculate_functionality_coverage()
        avg_functionality = sum(func_coverage.values()) / len(func_coverage)

        # Calculate overall progress (weighted average)
        overall_progress = (avg_functionality * 0.6 + lcr * 0.2 + mir * 0.2)

        report = []
        report.append("=" * 60)
        report.append("GLANG SELF-HOSTING METRICS")
        report.append("=" * 60)
        report.append("")
        report.append(f"Overall Self-Hosting Progress: {overall_progress:.1f}%")
        report.append("")

        report.append("LINES OF CODE:")
        report.append(f"  Python Backend:     {python_lines:,} lines")
        report.append(f"  Glang Stdlib:       {glang_stdlib_lines:,} lines")
        report.append(f"  Glang Samples:      {glang_samples_lines:,} lines")
        report.append(f"  Lines Code Ratio:   {lcr:.1f}%")
        report.append("")

        report.append("MODULE CLASSIFICATION:")
        report.append(f"  Total Modules:      {len(modules)}")
        report.append(f"  Pure Glang:         {len(pure_modules)} ({len(pure_modules)/len(modules)*100:.1f}%)")
        report.append(f"  Hybrid:             {len(hybrid_modules)} ({len(hybrid_modules)/len(modules)*100:.1f}%)")
        report.append(f"  Wrapper:            {len(wrapper_modules)} ({len(wrapper_modules)/len(modules)*100:.1f}%)")
        report.append("")

        report.append("Pure Glang Modules:")
        for m in sorted(pure_modules):
            report.append(f"  ✓ {m}.gr ({modules[m]['lines']} lines)")
        report.append("")

        report.append("Hybrid Modules:")
        for m in sorted(hybrid_modules):
            report.append(f"  ◐ {m}.gr ({modules[m]['lines']} lines)")
        report.append("")

        report.append("Wrapper Modules:")
        for m in sorted(wrapper_modules):
            report.append(f"  ○ {m}.gr ({modules[m]['lines']} lines)")
        report.append("")

        report.append("FUNCTIONALITY COVERAGE:")
        for category, percent in sorted(func_coverage.items(), key=lambda x: x[1], reverse=True):
            bar = self.progress_bar(percent)
            report.append(f"  {category:20} {percent:3.0f}%  {bar}")
        report.append("")

        report.append("KEY METRICS:")
        report.append(f"  Functionality Self-Hosting (FSR): {avg_functionality:.1f}%")
        report.append(f"  Lines of Code Ratio (LCR):        {lcr:.1f}%")
        report.append(f"  Module Independence Ratio (MIR):  {mir:.1f}%")
        report.append("")

        report.append("SELF-HOSTING LEVEL: 0 (Hosted Language)")
        report.append("  ✗ Execution engine in host language")
        report.append("  ✗ Parser/lexer in host language")
        report.append("  ◐ Standard library mixed")
        report.append("")

        report.append("NEXT MILESTONES:")
        report.append("  [ ] Tree/Graph data structures (pure Glang)")
        report.append("  [ ] Statistics module (pure Glang)")
        report.append("  [ ] Testing framework (pure Glang)")
        report.append("  [ ] Begin Rust migration bootstrap")

        return "\n".join(report)

    def progress_bar(self, percent: float, width: int = 25) -> str:
        """Generate a text progress bar."""
        filled = int(width * percent / 100)
        empty = width - filled
        return "█" * filled + "░" * empty

    def save_json_metrics(self, output_file: str = "metrics.json"):
        """Save metrics in JSON format for tracking."""
        python_lines = self.count_lines(self.python_src, "py")
        glang_stdlib_lines = self.count_lines(self.stdlib, "gr")

        modules = self.analyze_modules()
        pure_count = len([m for m, d in modules.items() if d['classification'] == 'pure'])

        from datetime import datetime
        metrics = {
            'timestamp': datetime.now().isoformat(),
            'directory': str(Path().cwd()),
            'lines_of_code': {
                'python': python_lines,
                'glang_stdlib': glang_stdlib_lines,
                'ratio_percent': round(glang_stdlib_lines / (python_lines + glang_stdlib_lines) * 100, 2)
            },
            'modules': {
                'total': len(modules),
                'pure': pure_count,
                'independence_ratio': round(pure_count / len(modules) * 100, 2) if modules else 0
            },
            'functionality_coverage': self.calculate_functionality_coverage(),
            'self_hosting_level': 0
        }

        with open(output_file, 'w') as f:
            json.dump(metrics, f, indent=2)

        return metrics


if __name__ == "__main__":
    metrics = SelfHostingMetrics()
    print(metrics.generate_report())

    # Also save JSON for tracking
    metrics.save_json_metrics("self_hosting_metrics.json")
    print("\nMetrics saved to self_hosting_metrics.json")