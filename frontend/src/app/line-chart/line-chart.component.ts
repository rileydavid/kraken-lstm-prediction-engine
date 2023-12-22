import { Component, OnChanges, Input, ElementRef, ViewChild, OnInit, AfterViewInit } from '@angular/core';
import * as d3 from 'd3';
import { DataService } from '../dataservice/data.service';
import { Ohlc } from '../models/ohlc.model';
import { CommonModule } from '@angular/common';


@Component({
  selector: 'app-line-chart',
  templateUrl: './line-chart.component.html',
  styleUrls: ['./line-chart.component.css'],
  standalone: true,
  imports: [CommonModule],
  providers: [DataService],
})
export class LineChartComponent implements OnInit {
  @ViewChild('chart')
  private chartContainer!: ElementRef;
  data: Ohlc[] = [];
  isLoading = false;

  constructor(private dataService: DataService) { }

  ngOnInit(): void {
    // fetch data
    this.dataService.fetchOhlcDayData().subscribe({
      next: (data) => {
        this.data = data;
        console.log(this.data);
        console.log("data fetched");
        this.isLoading = false;
        this.createChart();
      },
      error: (error) => {
        console.error('There was an error!', error);
      }
    });
  }

  private createChart(): void {
    console.log("createChart");
    if (!this.data) return;
    d3.select(this.chartContainer.nativeElement).selectAll("*").remove();
  
    // Define margins
    const margin = { top: 20, right: 30, bottom: 30, left: 50 };
    const width = 800 - margin.left - margin.right;
    const height = 400 - margin.top - margin.bottom;
  
    // Append the svg object to the body of the page
    const svg = d3.select(this.chartContainer.nativeElement)
      .append('svg')
      .attr('width', width + margin.left + margin.right)
      .attr('height', height + margin.top + margin.bottom)
      .append('g')
      .attr('transform', `translate(${margin.left},${margin.top})`);
  
    
    // Set the ranges
    const xScale = d3.scaleTime()
      .domain(d3.extent(this.data, (d: Ohlc) => d.bucket) as [Date, Date])
      .range([0, width]);
  
    // Find the min and max close_price values
    const minPrice = d3.min(this.data, d => d.close_price) ?? 0;
    const maxPrice = d3.max(this.data, d => d.close_price) ?? 0;

    // Calculate padding
    const padding = (maxPrice - minPrice) * 0.1; // 10% padding

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
      .datum(this.data)
      .attr("fill", "none")
      .attr("stroke", "steelblue")
      .attr("stroke-width", 1.5)
      .attr("d", line);
  
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