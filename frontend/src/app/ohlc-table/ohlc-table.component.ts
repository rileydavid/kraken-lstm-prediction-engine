import { Component, Input, OnChanges, SimpleChanges, ViewChild } from '@angular/core';
import { Ohlc } from '../models/ohlc.model';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatSort, MatSortModule } from '@angular/material/sort';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-ohlc-table',
  standalone: true,
  imports: [CommonModule, MatTableModule, MatSortModule],
  templateUrl: './ohlc-table.component.html',
  styleUrls: ['./ohlc-table.component.css'] // Corrected from 'styleUrl' to 'styleUrls'
})
export class OhlcTableComponent {
  @Input() data: Ohlc[] = [];

  displayedColumns: string[] = ['bucket', 'close_price', 'volume', 'count'];

  constructor() { }

}
