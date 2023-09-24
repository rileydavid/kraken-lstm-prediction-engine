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

  latest_price: string;
  predictions: string[];

  ngOnInit(){
    this.interval = setInterval(() => { 
        this.refreshData(); 
    }, 10000);
    
  }

  mapData(data: any): any {
    const timestamp = new Date(data.time);
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

      //console.log("here", tradesData);
      
      tradesData = tradesData.sort((a: any, b: any) => {
        return a.x < b.x;
      });

      this.latest_price = JSON.stringify(tradesData.slice(-1));

      this.series = [{
        name: "XBTUSD",
        data: tradesData
      }];    
    });


    this.dataService.getPrediction("15", "XBTUSD").subscribe((data: any) => {
      this.predictions[0] = data;
    });


    this.dataService.getPrediction("30", "XBTUSD").subscribe((data: any) => {
      this.predictions[1] = data;
    });


    this.dataService.getPrediction("60", "XBTUSD").subscribe((data: any) => {
      this.predictions[2] = data;
    });

  }

  constructor(private dataService: DataService) {
    this.predictions = ["", "", ""];
    this.latest_price = "";

    this.series = [{
      name: "XBTUSD",
      data: []
    }];    

    this.refreshData();
    
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