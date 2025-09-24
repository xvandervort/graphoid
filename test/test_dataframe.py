"""Tests for DataFrame module - tabular data as governed graphs."""

import pytest
from test_base import TestBase


class TestDataFrameBasics(TestBase):
    """Test basic DataFrame creation and operations."""

    def test_create_empty_dataframe(self):
        """Test creating an empty DataFrame."""
        result = self.execute("""
            import "dataframe" as df
            frame = df.create(["name", "age", "city"])
            frame["_columns"]
        """)
        assert result.to_python() == ["name", "age", "city"]

    def test_dataframe_from_records(self):
        """Test creating DataFrame from records."""
        result = self.execute("""
            import "dataframe" as df

            records = [
                { "name": "Alice", "age": 30, "city": "NYC" },
                { "name": "Bob", "age": 25, "city": "LA" },
                { "name": "Charlie", "age": 35, "city": "Chicago" }
            ]

            frame = df.from_records(records)
            frame["_row_count"]
        """)
        assert result.to_python() == 3

    def test_dataframe_add_row(self):
        """Test adding rows to DataFrame."""
        result = self.execute("""
            import "dataframe" as df

            frame = df.create(["name", "score"])
            df.add_row(frame, { "name": "Alice", "score": 95 })
            df.add_row(frame, { "name": "Bob", "score": 87 })

            frame["score"]
        """)
        assert result.to_python() == [95, 87]

    def test_dataframe_select_columns(self):
        """Test selecting specific columns."""
        result = self.execute("""
            import "dataframe" as df

            records = [
                { "name": "Alice", "age": 30, "city": "NYC", "score": 95 },
                { "name": "Bob", "age": 25, "city": "LA", "score": 87 }
            ]

            frame = df.from_records(records)
            subset = df.select(frame, ["name", "score"])
            subset["_columns"]
        """)
        assert result.to_python() == ["name", "score"]

    def test_dataframe_filter(self):
        """Test filtering DataFrame rows."""
        result = self.execute("""
            import "dataframe" as df

            records = [
                { "name": "Alice", "score": 95 },
                { "name": "Bob", "score": -10 },
                { "name": "Charlie", "score": 87 }
            ]

            frame = df.from_records(records)
            positive = df.filter(frame, "score", "positive")
            positive["_row_count"]
        """)
        assert result.to_python() == 2

    def test_dataframe_aggregate_sum(self):
        """Test sum aggregation."""
        result = self.execute("""
            import "dataframe" as df

            records = [
                { "product": "A", "sales": 100 },
                { "product": "B", "sales": 200 },
                { "product": "C", "sales": 150 }
            ]

            frame = df.from_records(records)
            total = df.aggregate(frame, "sales", "sum")
            total
        """)
        assert result.to_python() == 450

    def test_dataframe_aggregate_mean(self):
        """Test mean aggregation."""
        result = self.execute("""
            import "dataframe" as df

            records = [
                { "name": "Alice", "score": 90 },
                { "name": "Bob", "score": 80 },
                { "name": "Charlie", "score": 85 }
            ]

            frame = df.from_records(records)
            avg = df.aggregate(frame, "score", "mean")
            avg
        """)
        assert result.to_python() == 85

    def test_dataframe_group_by(self):
        """Test group by operation."""
        result = self.execute("""
            import "dataframe" as df

            records = [
                { "dept": "Sales", "salary": 50000 },
                { "dept": "Sales", "salary": 60000 },
                { "dept": "Engineering", "salary": 80000 },
                { "dept": "Engineering", "salary": 90000 }
            ]

            frame = df.from_records(records)
            grouped = df.group_by(frame, "dept", "salary", "mean")
            grouped["Sales"]
        """)
        assert result.to_python() == 55000

    def test_dataframe_head(self):
        """Test getting first n rows."""
        result = self.execute("""
            import "dataframe" as df

            records = []
            for i in [].upto(10) {
                records.append({ "id": i, "value": i * 10 })
            }

            frame = df.from_records(records)
            top3 = df.head(frame, 3)
            top3["_row_count"]
        """)
        assert result.to_python() == 3

    def test_dataframe_to_csv(self):
        """Test converting DataFrame to CSV."""
        result = self.execute("""
            import "dataframe" as df

            records = [
                { "name": "Alice", "age": 30 },
                { "name": "Bob", "age": 25 }
            ]

            frame = df.from_records(records)
            csv = df.to_csv(frame)
            csv
        """)
        expected = "name,age\nAlice,30\nBob,25"
        assert result.to_python() == expected


class TestDataFrameWithCSV(TestBase):
    """Test DataFrame CSV integration."""

    def test_dataframe_from_csv(self):
        """Test creating DataFrame from CSV text."""
        result = self.execute("""
            import "dataframe" as df

            csv_text = "name,score\\nAlice,95\\nBob,87"
            frame = df.from_csv(csv_text, true)
            frame["score"]
        """)
        # Note: CSV parser returns strings, not numbers yet
        assert result.to_python() == ["95", "87"]

    def test_dataframe_from_csv_no_headers(self):
        """Test creating DataFrame from CSV without headers."""
        result = self.execute("""
            import "dataframe" as df

            csv_text = "Alice,95\\nBob,87"
            frame = df.from_csv(csv_text, false)
            frame["_columns"]
        """)
        assert result.to_python() == ["col0", "col1"]


class TestDataFrameGovernance(TestBase):
    """Test DataFrame governance concepts."""

    def test_dataframe_rules_definition(self):
        """Test that DataFrame defines its governance rules."""
        result = self.execute("""
            import "dataframe" as df

            rules = df._define_dataframe_rules()
            rules.size()
        """)
        assert result.to_python() == 5  # 5 governance rules defined

    def test_dataframe_type_marker(self):
        """Test that DataFrames are marked with type."""
        result = self.execute("""
            import "dataframe" as df

            frame = df.create(["a", "b"])
            frame["_type"]
        """)
        assert result.to_python() == "dataframe"