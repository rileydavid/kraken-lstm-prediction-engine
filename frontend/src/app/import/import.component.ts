import { Component, OnInit } from '@angular/core';
import { DataService } from '../dataservice/data.service';
import { ImportFile } from '../models/import-file';
import { CommonModule } from '@angular/common';
import { MatTableModule } from '@angular/material/table';
import { MatSortModule } from '@angular/material/sort';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatInputModule } from '@angular/material/input';


@Component({
  selector: 'app-import',
  standalone: true,
  imports: [CommonModule, FormsModule, ReactiveFormsModule, MatTableModule, MatSortModule, MatFormFieldModule, MatProgressSpinnerModule, MatInputModule],
  templateUrl: './import.component.html',
  styleUrl: './import.component.css',
  providers: [DataService]
})
export class ImportComponent implements OnInit {

  constructor(private dataService: DataService) { }

  isLoading = true;
  isImporting = false;
  displayedColumns: string[] = ['file_name', 'file_size', 'uploaded', 'symbol', 'import'];
  files: ImportFile[] = [];

  ngOnInit(): void {
    this.dataService.fetchImportFiles().subscribe(data => {
      this.files = data;
      console.log(data);
      this.isLoading = false;
    });
  }


  onImportClick(file: ImportFile): void {
    // Define your action here. Example:
    console.log(`Import started for ${file.file_name}, ${file.symbol}`);
    alert(`Import started for ${file.file_name}`);
    this.isImporting = true;
    this.dataService.importFile(file).subscribe(data => {
      console.log(data);
      if(data.toString() == "Ok"){
        // remove file from list
        this.files = this.files.filter(item => item.file_name !== file.file_name);
        alert(`Import finished for ${file.file_name}`);
      }else {
        alert(`Import failed for ${file.file_name}`);
      }
      this.isImporting = false;
    });;

  }

  trackByFn(index: any, item: any) {
    return item.file_name; // or any unique identifier of your items
  }
  
}
