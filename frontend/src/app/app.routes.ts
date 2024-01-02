import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { DashboardComponent } from './dashboard/dashboard.component';
import { ConfigComponent } from './config/config.component';
import { TradesComponent } from './trades/trades.component';
import { UploadComponent } from './upload/upload.component';
import { ImportComponent } from './import/import.component';

export const routes: Routes = [
    { path: 'dashboard', component: DashboardComponent },
    { path: 'config', component: ConfigComponent },
    { path: 'trades', component: TradesComponent},
    { path: 'upload', component: UploadComponent },
    { path: 'import', component: ImportComponent },
    { path: '', redirectTo: '/dashboard', pathMatch: 'full' },
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})
export class AppRoutingModule { }