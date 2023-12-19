package tests

import (
	"testing"

	"github.com/OdinSrc/advent-of-code/2023/golang/day19/cmd/part1"
)

func TestPartOne(t *testing.T) {
	result, err := part1.SolvePartOne("../../input_test.txt")
	if err != nil {
		t.Fatal(err)
	}
	expected := "19114"
	if *result != expected {
		t.Fatalf("Got %s, wanted %s", *result, expected)
	}
}
