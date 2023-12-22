import { Component, OnInit } from '@angular/core';
import { LineChartComponent } from '../line-chart/line-chart.component';
import { DataService } from '../dataservice/data.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.css',
  standalone: true,
  imports: [LineChartComponent]
})
export class DashboardComponent implements OnInit{

  constructor() {

  }

  ngOnInit(): void {
    
  }

}
