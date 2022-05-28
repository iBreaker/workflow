/*
* @author: YangRui
*  @date: 2022/05/28 21:07
 */

package main

import (
	"time"

	"fyne.io/fyne/v2"
	"fyne.io/fyne/v2/app"
	"fyne.io/fyne/v2/container"
	"fyne.io/fyne/v2/data/binding"
	"fyne.io/fyne/v2/widget"
)

const minutes = 45

func main() {
	myApp := app.New()
	myWindow := myApp.NewWindow("Breaker's Workflow")

	progressValue := binding.NewFloat()

	progress := widget.NewProgressBar()
	progress.Bind(progressValue)
	progress.Max = minutes * 60
	progress.TextFormatter = func() string {
		vv, _ := progressValue.Get()
		v := time.Duration(int(vv))
		return time.Duration(v * time.Second).String()
	}

	go func() {
		c := time.NewTicker(time.Second).C
		for range c {
			v, _ := progressValue.Get()
			if v > progress.Max {
				continue
			}
			progressValue.Set(v + 1)
		}
	}()

	buttonContent := widget.NewButton("restart", func() {
		progressValue.Set(0)
	})

	myWindow.SetContent(container.NewVBox(progress, buttonContent))
	myWindow.Resize(fyne.NewSize(400, 200))
	myWindow.ShowAndRun()
}
