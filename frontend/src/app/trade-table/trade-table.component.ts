import { Component, Input } from '@angular/core';
import { MatTableModule } from '@angular/material/table';
import { MatSortModule } from '@angular/material/sort';
import { CommonModule } from '@angular/common';
import { Trade } from '../models/trade.model';

@Component({
  selector: 'app-trade-table',
  standalone: true,
  imports: [CommonModule, MatTableModule, MatSortModule],
  templateUrl: './trade-table.component.html',
  styleUrls: ['./trade-table.component.css'] // Corrected from 'styleUrl' to 'styleUrls'
})
export class TradeTableComponent {
  @Input() data: Trade[] = [];

  displayedColumns: string[] = ['time', 'price', 'volume', 'side', 'order_type', 'symbol_id'];

  constructor() { }

}
