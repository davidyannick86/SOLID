package solid

import "fmt"

type Printer interface {
	Print()
}

type Scanner interface {
	Scan()
}

type SimplePrinter struct{}

func (sp *SimplePrinter) Print() {
	fmt.Println("Printing...")
}

type MultiFunctionPrinter struct{}

func (mfp *MultiFunctionPrinter) Print() {
	fmt.Println("Printing with multifunction printer...")
}

func (mfp *MultiFunctionPrinter) Scan() {
	fmt.Println("Scanning...")
}

func UsePrinter(p Printer) {
	p.Print()
}
