package main

import (
	_ "embed"
	"testing"
)

func TestPart1Input(t *testing.T) {
	result := Part1(inputDay)
	expected := 112815
	if result != expected {
		t.Errorf("Result is incorrect, got: %d, want: %d.", result, expected)
	}
}

//func TestPart2(t *testing.T) {
//	result := Part2(inputTest)
//	expected := 2713310158
//	if result != expected {
//		t.Errorf("Result is incorrect, got: %d, want: %d.", result, expected)
//	}
//}

func TestPart2Input(t *testing.T) {
	result := Part2(inputDay)
	expected := 25738411485
	if result != expected {
		t.Errorf("Result is incorrect, got: %d, want: %d.", result, expected)
	}
}

func BenchmarkPart1(b *testing.B) {
	for range b.N {
		Part1(inputDay)
	}
}
func BenchmarkPart2(b *testing.B) {
	for range b.N {
		Part2(inputDay)
	}
}
