import { HttpClient, HttpXhrBackend } from "@angular/common/http";
import { Component, OnInit, ViewChild } from "@angular/core";
import { DataService } from "./networking/DataService";

import {
  ApexAxisChartSeries,
  ApexChart,
  ApexTitleSubtitle,
  ApexDataLabels,
  ApexFill,
  ApexMarkers,
  ApexYAxis,
  ApexXAxis,
} from "ng-apexcharts";
import { Trade } from "./model/trade";
import { elementAt } from "rxjs";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"]
})

export class AppComponent implements OnInit {
  
  interval: any;
  chart: ApexChart;
  dataLabels: ApexDataLabels;
  markers: ApexMarkers;
  title: ApexTitleSubtitle;
  fill: ApexFill;
  yaxis: ApexYAxis;
  xaxis: ApexXAxis;
  series: ApexAxisChartSeries;

  ngOnInit(){
    this.refreshData();
 
    
    this.interval = setInterval(() => { 
        this.refreshData(); 
    }, 100000);
    
  }

  mapData(data: any): any {
    const timestamp = data.time;
    const price = parseFloat(data.price);
    return { x: timestamp, y: price };
  }


  // performance is not so good yet maybe it would be better to append data instead of rebuilding
  refreshData(): any {
    this.dataService.getTrades("60", "XBTUSD").subscribe((data: any) => {
      
      let tradesData: any = []; 

      data.map((item: any) => {
        tradesData.push(this.mapData(item));
      })

      console.log("here", tradesData[0]);
      
      tradesData.sort((a: any, b: any) => {
        return new Date(a.x) > new Date(b.x);
      });

      this.series = [{
        name: "XBTUSD",
        data: tradesData
      }];    
    });
  }

  constructor(private dataService: DataService) {
    this.series = [{
      name: "XBTUSD",
      data: []
    }];    

    this.chart = {
      type: "area",
      stacked: false,
      zoom: {
        type: "x",
        enabled: true,
        autoScaleYaxis: true
      },
      toolbar: {
        autoSelected: "zoom"
      }
    };

    this.dataLabels = {
      enabled: false
    };

    this.markers = {
      size: 0
    };

    this.title = {
      text: "XBTUSD",
      align: "left"
    };

    this.fill = {
      type: "gradient",
      gradient: {
        shadeIntensity: 1,
        inverseColors: false,
        opacityFrom: 0.5,
        opacityTo: 0,
        stops: [0, 90, 100]
      }
    };

    this.yaxis = {
      labels: {
        
      }, 
      title: {
        text: "Price"
      }
    };

    this.xaxis = {
      type: "category",
      tickAmount: 15
    };
  }
}