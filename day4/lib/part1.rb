require_relative './common'

def peak_sleep(lines)
  log_parser = LogParser.new
  log = Log.new

  lines.map { |line| log_parser.parse_line(line) }
       .compact
       .each { |event| log.add_event(event) }

  log.peak_sleep
end

def main(filename)
  peak_sleep(File.open(filename).readlines.sort)
end

