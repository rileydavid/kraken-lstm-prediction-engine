import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { environment } from '../../environments/environments';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';

@Component({
  selector: 'app-upload',
  standalone: true,
  imports: [
    MatFormFieldModule,
    MatInputModule,
    MatProgressSpinnerModule,
  ],
  templateUrl: './upload.component.html',
  styleUrl: './upload.component.css',
})
export class UploadComponent {

  constructor(private http: HttpClient) { }

  isLoading = false;
  files: File[] = [];
  fileNames = '';

  onFilesSelected(event: any) {
    this.files = event.target.files;
    this.fileNames = Array.from(this.files).map(file => file.name).join(', ');
  }

  uploadFiles() {
    if (this.files.length === 0) return;

    this.isLoading = true;
    const formData = new FormData();
    Array.from(this.files).forEach(file => formData.append('files', file, file.name));

    this.http.post(environment.apiUrl + '/upload', formData).subscribe(response => {
      this.isLoading = false;
      console.log('Upload success', response);
      // Clear the files after upload
      this.files = [];
      this.fileNames = '';
      this.isLoading = false;
    }, error => {
      this.isLoading = false;
      console.error('Upload error', error);
    });
  }
}


