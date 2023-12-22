import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterLink, RouterOutlet } from '@angular/router';
import { NavbarComponent } from "./navbar/navbar.component";
import { DashboardComponent } from './dashboard/dashboard.component';
import { ConfigComponent } from './config/config.component';
import { AppRoutingModule } from './app.routes';
import { LineChartComponent } from './line-chart/line-chart.component';
import { DataService } from './dataservice/data.service';
import { HttpClient, HttpClientModule } from '@angular/common/http';

@Component({
    selector: 'app-root',
    standalone: true,
    templateUrl: './app.component.html',
    styleUrl: './app.component.css',
    imports: [CommonModule, RouterLink, RouterOutlet, NavbarComponent, HttpClientModule],
    providers: [DataService]
})
export class AppComponent {
  title = 'frontend';
}
