import { Component, OnInit, ViewChild } from '@angular/core';
import { LineChartComponent } from '../line-chart/line-chart.component';
import { DataService } from '../dataservice/data.service';
import { DatepickerComponent } from '../datepicker/datepicker.component';
import { SymbolpickerComponent } from '../symbolpicker/symbolpicker.component';
import { Ohlc } from '../models/ohlc.model';
import { SymbolModel } from '../models/symbol.model';
import { forkJoin } from 'rxjs';
import { OhlcTableComponent } from '../ohlc-table/ohlc-table.component';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.css',
  standalone: true,
  imports: [LineChartComponent, DatepickerComponent, SymbolpickerComponent, OhlcTableComponent],
  providers: [DataService],
})
export class DashboardComponent implements OnInit {
  @ViewChild('lineChartRef')
  linechart: LineChartComponent = new LineChartComponent;

  data: Ohlc[] = [];
  predictedData: Ohlc[] = [];
  fromDate: Date = new Date();
  toDate: Date = new Date();
  selectedSymbol: SymbolModel = { "id": 0, "symbol": "" };
  selected: boolean = false;
  loadingData: boolean = false;
  dataLoaded: boolean = false;
  symbols: SymbolModel[] = [];

  ngOnInit(): void {
    this.dataService.fetchSymbols().subscribe({
      next: (data) => {
        this.symbols = data;
        console.log("Fetched Symbols")
      },
      error: (error) => {
        console.error('There was an error whilst fetching symbols!', error);
      }
    });
  }

  onFromDateChange(date: Date): void {
    this.fromDate = date;
  }

  onToDateChange(date: Date): void {
    this.toDate = date;
  }

  constructor(private dataService: DataService) { }

  onButtonClick(): void {
    console.log("fetch");

    if (this.fromDate > this.toDate) {
      alert("From date must be before to date");
      return;
    }

    if (this.selectedSymbol.id == 0) {
      alert("Please select a symbol");
      return;
    }

    this.selected = true;
    this.loadingData = true;

    const ohlcHourRangeData$ = this.dataService.fetchOhlcHourRangeData(this.fromDate, this.toDate, this.selectedSymbol.id);
    // Create an observable for the second data fetch
    const ohlcPrediction$ = this.dataService.fetchOhlcPrediction(this.selectedSymbol.id, this.toDate);

    // Use forkJoin to wait for both requests to complete
    forkJoin([ohlcHourRangeData$, ohlcPrediction$]).subscribe({
      next: ([ohlcData, predictionData]) => {
        this.data = ohlcData
        console.log(predictionData)
        this.data.push(predictionData);
        this.loadingData = false;
        this.dataLoaded = true; 
        this.linechart.createChart(this.data);
      },
      error: (error) => {
        console.error('There was an error!', error);
      }
    });
   
  }

  handleSymbolEvent(data: SymbolModel) {
    this.selectedSymbol = data;
    this.selected = true;
  }
}
