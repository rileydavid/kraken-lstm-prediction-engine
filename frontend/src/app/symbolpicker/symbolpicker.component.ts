import { ChangeDetectionStrategy, Component, EventEmitter, OnInit, Output } from '@angular/core';
import { DataService } from '../dataservice/data.service';
import { SymbolModel } from '../models/symbol.model';
import { CommonModule } from '@angular/common';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatSelectModule } from '@angular/material/select';

@Component({
  selector: 'app-symbolpicker',
  standalone: true,
  imports: [CommonModule, FormsModule, ReactiveFormsModule, MatSelectModule],
  templateUrl: './symbolpicker.component.html',
  styleUrl: './symbolpicker.component.css',
  providers: [DataService],
})
export class SymbolpickerComponent implements OnInit {
  @Output() selectedSymbolEvent = new EventEmitter<SymbolModel>;
  selectedSymbol: SymbolModel = { "id": 0, "symbol": "" };  //Placeholder 

  symbols: SymbolModel[] = [];

  constructor(private dataService: DataService) { }

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

  onSymbolSelected($event: any): void {
    console.log(this.selectedSymbol);
    this.selectedSymbolEvent.emit(this.selectedSymbol);
  }
}
