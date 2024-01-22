import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { environment } from '../../environments/environments';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { DataService } from '../dataservice/data.service';

@Component({
  selector: 'app-upload',
  standalone: true,
  imports: [
    MatFormFieldModule,
    MatInputModule,
    MatProgressSpinnerModule,
  ],
  providers: [DataService],
  templateUrl: './upload.component.html',
  styleUrl: './upload.component.css',
})
export class UploadComponent implements OnInit {

  constructor(private dataService: DataService) { }

  ngOnInit(): void {}

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


    this.dataService.uploadFiles(formData).subscribe(data => {
      if(data.toString() == "Ok"){
        // remove file from list
        this.files = [];
        this.fileNames = '';
        this.isLoading = false;
        alert(`Import success`);
      }else {
        alert(`Import failed`);
      }
      this.isLoading = false;
    });
  }
}


