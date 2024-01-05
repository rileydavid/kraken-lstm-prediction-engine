import { Component, OnInit } from '@angular/core';
import { DataService } from '../dataservice/data.service';
import { SymbolModel } from '../models/symbol.model';
import { SymbolpickerComponent } from '../symbolpicker/symbolpicker.component';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

@Component({
  selector: 'app-subscription',
  standalone: true,
  imports: [SymbolpickerComponent, MatProgressSpinnerModule],
  templateUrl: './subscription.component.html',
  styleUrl: './subscription.component.css'
})
export class SubscriptionComponent implements OnInit {
  loadingData: boolean = true;
  selectedSymbol: SymbolModel = { "id": 0, "symbol": "" };
  symbols: SymbolModel[] = [];

  constructor(private dataService: DataService) { }

  ngOnInit(): void {
    this.dataService.fetchAvailableSymbols().subscribe({
      next: (data) => {
        this.symbols = data;
        this.loadingData = false;
        console.log("Fetched Symbols")
      },
      error: (error) => {
        console.error('There was an error whilst fetching symbols!', error);
      }
    });
  }

  handleSymbolEvent(data: SymbolModel) {
    this.selectedSymbol = data;
  }

  onButtonClick() {
    this.loadingData = true;
    this.dataService.addSubscription(this.selectedSymbol.symbol).subscribe({
      next: (data) => {
        this.loadingData = false;
        alert("Added Subscription for " + this.selectedSymbol.symbol);
        console.log("Added Subscription")
      },
      error: (error) => {
        console.error('There was an error whilst adding subscription!', error);
      }
    });
  
  }
}
