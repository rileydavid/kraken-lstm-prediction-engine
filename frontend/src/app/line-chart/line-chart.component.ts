import { Component, OnChanges, Input, ElementRef, ViewChild, OnInit, AfterViewInit, ChangeDetectorRef } from '@angular/core';
import * as d3 from 'd3';
import { DataService } from '../dataservice/data.service';
import { Ohlc } from '../models/ohlc.model';
import { CommonModule } from '@angular/common';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

@Component({
  selector: 'app-line-chart',
  templateUrl: './line-chart.component.html',
  styleUrls: ['./line-chart.component.css'],
  standalone: true,
  imports: [CommonModule, MatProgressSpinnerModule],
  providers: [DataService],
})
export class LineChartComponent implements OnInit {
  @Input() selected: boolean = false;
  @Input() loadingData: boolean = false;

  @ViewChild('chart', { static: false })
  private chartContainer!: ElementRef;

  constructor() { }

  ngOnInit(): void { }

  public createChart(data: Ohlc[]): void {
    console.log("createChart");
    if (!data) return;

    const legendData = [
      { color: "steelblue", label: "Actual" },
      { color: "orange", label: "Prediction" }
    ];

    const container = this.chartContainer.nativeElement;
    d3.select(container).selectAll("*").remove();

    // margins
    const margin = { top: 20, right: 30, bottom: 40, left: 70 };
    const width = 800 - margin.left - margin.right;
    const height = 600 - margin.top - margin.bottom;

    const svg = d3.select(this.chartContainer.nativeElement)
      .append('svg')
      .attr('width', width + margin.left + margin.right)
      .attr('height', height + margin.top + margin.bottom)
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);

    const legendGroup = svg.append("g")
      .attr("class", "legend-group")
      .attr("transform", `translate(${width - 150}, 20)`); // Adjust this to fit the legend inside your SVG


    // Position the legend
    legendGroup.selectAll(".legend-item")
      .data(legendData)
      .enter().append("g")
      .attr("class", "legend-item")
      .attr("transform", (d, i) => `translate(0, ${i * 25})`) // Space out legend items vertically
      .each(function (d) {
        d3.select(this).append("rect")
          .attr("width", 20)
          .attr("height", 20)
          .attr("fill", d.color);

        d3.select(this).append("text")
          .attr("x", 25)
          .attr("y", 15) // Adjust for vertical alignment with the box
          .text(d.label);
      });

    // Set the ranges
    const xScale = d3.scaleTime()
      .domain(d3.extent(data, (d: Ohlc) => d.bucket) as [Date, Date])
      .range([0, width]);

    // Find the min and max close_price values
    const minPrice = d3.min(data, d => d.close_price) ?? 0;
    const maxPrice = d3.max(data, d => d.close_price) ?? 0;

    // Calculate padding
    const padding = (maxPrice - minPrice) * 0.5;

    // Adjust the yScale domain with the new padded values
    const yScale = d3.scaleLinear()
      .domain([minPrice - padding, maxPrice + padding])
      .range([height, 0]);

    // Define the line
    const line = d3.line<Ohlc>()
      .x((d: Ohlc) => xScale(d.bucket))
      .y((d: Ohlc) => yScale(d.close_price));

    // Add the line path.
    svg.append("path")
      .datum(data.slice(0, -1))
      .attr("fill", "none")
      .attr("stroke", "steelblue")
      .attr("stroke-width", 1.5)
      .attr("d", line);

    const lastPointLine = d3.line<Ohlc>()
      .x((d: Ohlc) => xScale(d.bucket))
      .y((d: Ohlc) => yScale(d.close_price));

    // Add the line path for the last data point in orange.
    if (data.length > 1) {
      svg.append("path")
        .datum(data.slice(-2)) // Use the last two points
        .attr("fill", "none")
        .attr("stroke", "orange")
        .attr("stroke-width", 1.5)
        .attr("d", lastPointLine);
    }

    // Add the X Axis
    svg.append("g")
      .attr("transform", `translate(0,${height})`)
      .call(d3.axisBottom(xScale).tickFormat((domainValue) => {
        return d3.timeFormat("%Y-%m-%d")(domainValue as Date);
      }));

    // Add the Y Axis
    svg.append("g")
      .call(d3.axisLeft(yScale));

    // Add the X Axis label
    svg.append("text")
      .attr("transform", `translate(${width / 2},${height + margin.bottom})`)
      .style("text-anchor", "middle")
      .text("Date");

    // Add the Y Axis label
    svg.append("text")
      .attr("transform", "rotate(-90)")
      .attr("y", 0 - margin.left)
      .attr("x", 0 - (height / 2))
      .attr("dy", "1em")
      .style("text-anchor", "middle")
      .text("Close Price");
  }
}