import { Module } from '@nestjs/common';
import { SplitModule } from './split/split.module';

@Module({
  imports: [SplitModule],
  controllers: [],
  providers: [],
})
export class AppModule {}
