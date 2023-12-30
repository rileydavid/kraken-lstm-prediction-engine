import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class FileUploadService {
  constructor(private http: HttpClient) {}

  uploadFile(file: File) {
    const formData: FormData = new FormData();
    formData.append('file', file, file.name);

    const headers = new HttpHeaders({
      'Content-Type': 'multipart/form-data'
    });

    // Replace with your server URL
    return this.http.post('http://localhost:3000/upload', formData, {
      headers: headers
    });
  }
}
