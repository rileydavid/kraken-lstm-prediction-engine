import { Component, ViewChild } from '@angular/core';
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
export class DashboardComponent {
  @ViewChild('lineChartRef')
  linechart: LineChartComponent = new LineChartComponent;

  data: Ohlc[] = [];
  predictedData: Ohlc[] = [];
  fromDate: Date = new Date();
  toDate: Date = new Date();
  selectedSymbol: SymbolModel = { "id": 0, "symbol": "" };
  selected: boolean = false;
  loadingData: boolean = false;

  onFromDateChange(date: Date): void {
    this.fromDate = date;
  }

  onToDateChange(date: Date): void {
    this.toDate = date;
  }

  constructor(private dataService: DataService) { }

  onButtonClick(): void {

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
        this.data.push(predictionData); // Assuming both are arrays, adjust if necessary
        this.loadingData = false;
        // Now that we have both sets of data, trigger the creation of the line chart
        this.linechart.createChart(this.data);
      },
      error: (error) => {
        console.error('There was an error!', error);
      }
    });
    /*
    this.dataService.fetchOhlcHourRangeData(this.fromDate, this.toDate, this.selectedSymbol.id).subscribe({
      next: (data) => {
        this.data = data;
        this.loadingData = false;
        // trigger the creation of linechart
        this.linechart.createChart(data);
      },
      error: (error) => {
        console.error('There was an error!', error);
      }
    });

    this.dataService.fetchOhlcPrediction(this.selectedSymbol.id, this.toDate).subscribe({
      next: (data) => {
        this.loadingData = false;
        this.data.push(data);
        console.log(data)
        // trigger the creation of linechart
      },
      error: (error) => {
        console.error('There was an error!', error);
      }
    });
    */
    
  }

  handleSymbolEvent(data: SymbolModel) {
    this.selectedSymbol = data;
    this.selected = true;
  }
}
