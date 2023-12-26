import { Component, OnInit } from '@angular/core';
import { LineChartComponent } from '../line-chart/line-chart.component';
import { DataService } from '../dataservice/data.service';
import { DatepickerComponent } from '../datepicker/datepicker.component';
import { SharedDateService } from '../shareddate/shareddate.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.css',
  standalone: true,
  imports: [LineChartComponent, DatepickerComponent],
  providers: [SharedDateService]
})
export class DashboardComponent implements OnInit{
  // give them some value for now
  startDate: Date = new Date();
  endDate: Date = new Date();

  onStartDateChange(date: Date): void {
    this.startDate = date;
  }

  onEndDateChange(date: Date): void {
    this.endDate = date;
  }


  constructor(private sharedDateService: SharedDateService) {}

  ngOnInit(): void {
    //throw new Error('Method not implemented.');
  }

  onButtonClick(): void {
    console.log("button clicked");
    console.log(this.startDate, this.endDate);
    // Perform your actions with startDate and endDate
  }
}
