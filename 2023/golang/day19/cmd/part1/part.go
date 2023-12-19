package part1

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type CMP int

const CMP_GREATER CMP = 1
const CMP_LESSER CMP = 2

type Instr struct {
	TargetRating   string
	Cmp            CMP
	CmpValue       int
	TargetWorkflow string
}

type Workflow struct {
	Label        string
	Instructions []Instr
}

func StrToWorkflow(input string) Workflow {
	ts := strings.Split(input, "{")
	instrs := strings.Replace(ts[1], "}", "", 1)

	instructions := []Instr{}
	for _, instr := range strings.Split(instrs, ",") {
		instrs := strings.Split(instr, ":")
		if len(instrs) > 1 {
			cmpInstr := instrs[0]
			targetWorkFlow := instrs[1]
			if strings.Contains(cmpInstr, "<") {
				cmpStatement := strings.Split(cmpInstr, "<")
				i, err := strconv.Atoi(strings.Replace(cmpStatement[1], "<", "", 1))
				if err != nil {
					log.Fatal("Invalid Data")
				}
				instr := Instr{
					TargetRating:   cmpStatement[0],
					CmpValue:       i,
					Cmp:            CMP_LESSER,
					TargetWorkflow: targetWorkFlow,
				}
				instructions = append(instructions, instr)
			} else {
				cmpStatement := strings.Split(cmpInstr, ">")
				i, err := strconv.Atoi(strings.Replace(cmpStatement[1], ">", "", 1))
				if err != nil {
					log.Fatal("Invalid Data")
				}
				instr := Instr{
					TargetRating:   cmpStatement[0],
					CmpValue:       i,
					Cmp:            CMP_GREATER,
					TargetWorkflow: targetWorkFlow,
				}
				instructions = append(instructions, instr)
			}
		} else {
			instructions = append(instructions, Instr{TargetWorkflow: instrs[0]})
		}
	}

	return Workflow{
		Label:        ts[0],
		Instructions: instructions,
	}
}

type Rating struct {
	x int
	m int
	a int
	s int
}

func strToInt(input string) int {
	i, _ := strconv.Atoi(input)
	return i
}

func StrToRating(input string) Rating {
	input1 := strings.Replace(input, "}", "", 1)
	input2 := strings.Replace(input1, "{", "", 1)

	rating := Rating{}
	for _, valueStr := range strings.Split(input2, ",") {
		value := strings.Split(valueStr, "=")
		switch value[0] {
		case "x":
			rating.x = strToInt(value[1])
			break
		case "m":
			rating.m = strToInt(value[1])
			break
		case "a":
			rating.a = strToInt(value[1])
			break
		case "s":
			rating.s = strToInt(value[1])
			break

		}

	}

	return rating
}

func SolvePartOne(inputFile string) (*string, error) {

	fileContent := getFileContent(inputFile)

	workflows, ratings := parse(fileContent)
	// fmt.Println(workflows, ratings)

	total := 0
	for _, rating := range ratings {
		value := doWorkFlow(workflows, rating)
		// fmt.Println(value)
		total += value
	}
	// fmt.Println(doWorkFlow(workflows, ratings[1]))

	fmt.Println("Total=", total)
	result := (fmt.Sprintf("%d", total))
	return &result, nil
}

func doWorkFlow(workflows map[string][]Instr, rating Rating) int {
	instrs := workflows["in"]
	var verdict *string = nil
	for verdict == nil {
	INSTR_LOOP:
		for _, instr := range instrs {
			// fmt.Println(instr)

			if instr.Cmp == 0 {
				if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
					verdict = &instr.TargetWorkflow
					break
				} else {
					instrs = workflows[instr.TargetWorkflow]
					break
				}

			}

			switch instr.TargetRating {
			case "x":
				if instr.Cmp == CMP_GREATER && rating.x > instr.CmpValue {
					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]
					break INSTR_LOOP
				}
				if instr.Cmp == CMP_LESSER && rating.x < instr.CmpValue {
					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]
					break INSTR_LOOP
				}
			case "m":
				if instr.Cmp == CMP_GREATER && rating.m > instr.CmpValue {
					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]
					break INSTR_LOOP
				}
				if instr.Cmp == CMP_LESSER && rating.m < instr.CmpValue {
					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]
					break INSTR_LOOP
				}
			case "a":
				if instr.Cmp == CMP_GREATER && rating.a > instr.CmpValue {
					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]
					break INSTR_LOOP
				}
				if instr.Cmp == CMP_LESSER && rating.a < instr.CmpValue {
					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]
					break INSTR_LOOP
				}
			case "s":
				if instr.Cmp == CMP_GREATER && rating.s > instr.CmpValue {

					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]
					break INSTR_LOOP
				}
				if instr.Cmp == CMP_LESSER && rating.s < instr.CmpValue {
					if instr.TargetWorkflow == "A" || instr.TargetWorkflow == "R" {
						verdict = &instr.TargetWorkflow
						break INSTR_LOOP
					}
					instrs = workflows[instr.TargetWorkflow]

					break INSTR_LOOP
				}

			}

		}

	}

	if *verdict == "A" {
		return rating.a + rating.m + rating.x + rating.s
	}

	return 0
}

func parse(input string) (map[string][]Instr, []Rating) {
	parts := strings.Split(input, "\n\n")

	wf_strs := strings.Split(parts[0], "\n")
	ratingsStr := strings.Split(parts[1], "\n")

	workflows := make(map[string][]Instr)
	for _, str := range wf_strs {
		workflow := StrToWorkflow(str)
		workflows[workflow.Label] = workflow.Instructions
	}

	ratings := []Rating{}
	for _, ratingStr := range ratingsStr {
		ratings = append(ratings, StrToRating(ratingStr))
	}

	return workflows, ratings
}

func getFileContent(inputFile string) string {

	b, err := os.ReadFile(inputFile)
	if err != nil {
		log.Fatal(err)
	}

	return string(b)
}
