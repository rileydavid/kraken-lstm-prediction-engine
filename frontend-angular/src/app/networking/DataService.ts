import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { Observable, throwError } from 'rxjs';
import { catchError, retry } from 'rxjs/operators';

import { HttpHeaders } from '@angular/common/http';
import { Trade } from '../model/trade';


@Injectable()
export class DataService {
    private baseUrl = "http://127.0.0.1:8080/";    

    constructor(private http: HttpClient) {}

    getTrades(interval: string, symbol: string) {
        return this.http.get<Trade>(this.baseUrl + "trades/" + interval + "/" + symbol);
    }
}