import { Component, ViewChild } from '@angular/core';
import { DataService } from '../dataservice/data.service';
import { DatepickerComponent } from '../datepicker/datepicker.component';
import { SymbolpickerComponent } from '../symbolpicker/symbolpicker.component';
import { SymbolModel } from '../models/symbol.model';
import { Trade } from '../models/trade.model';
import { TradeTableComponent } from '../trade-table/trade-table.component';
import { symbol } from 'd3';

@Component({
  selector: 'app-trades',
  standalone: true,
  templateUrl: './trades.component.html',
  styleUrl: './trades.component.css',
  imports: [SymbolpickerComponent, DatepickerComponent, TradeTableComponent]
})
export class TradesComponent {
  data: Trade[] = [];
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


    this.dataService.fetchTrades(this.fromDate, this.toDate, this.selectedSymbol.id).subscribe({
      next: (data) => {
        this.data = data;
        console.log("Fetched Symbols")
      },
      error: (error) => {
        console.error('There was an error whilst fetching symbols!', error);
      }
    });
  }
  
  handleSymbolEvent(data: SymbolModel) {
    this.selectedSymbol = data;
    this.selected = true;
  }
}
