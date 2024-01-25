import { Component, EventEmitter, Input, Output } from '@angular/core';
import { DataService } from '../dataservice/data.service';
import { SymbolModel } from '../models/symbol.model';
import { CommonModule } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatSelectModule } from '@angular/material/select';

@Component({
  selector: 'app-symbolpicker',
  standalone: true,
  imports: [CommonModule, FormsModule, ReactiveFormsModule, MatSelectModule],
  templateUrl: './symbolpicker.component.html',
  styleUrl: './symbolpicker.component.css',
  providers: [DataService],
})
export class SymbolpickerComponent {
  @Input() symbols: SymbolModel[] = [];
  @Output() selectedSymbolEvent = new EventEmitter<SymbolModel>;
  selectedSymbol: SymbolModel = { "id": 0, "symbol": "" };  //Placeholder 
  
  constructor() { }

  onSymbolSelected($event: any): void {
    console.log(this.selectedSymbol);
    this.selectedSymbolEvent.emit(this.selectedSymbol);
  }
}
