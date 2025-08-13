import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Badge } from "@/components/ui/badge";
import {
  Zap,
  Globe,
  Code,
  Users,
  Star,
  ArrowRight,
  Github,
  Twitter,
  Linkedin,
} from "lucide-react";

export default function Home() {
  return (
    <div className="min-h-screen bg-white">
      {/* Header */}
      <header className="border-b border-gray-200 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center space-x-2">
              <div className="w-8 h-8 bg-black rounded-lg flex items-center justify-center">
                <span className="text-white font-bold text-lg">V6</span>
              </div>
              <span className="text-black font-bold text-xl">V6</span>
            </div>
            <nav className="hidden md:flex space-x-8">
              <a
                href="#features"
                className="text-gray-600 hover:text-black transition-colors"
              >
                Features
              </a>
              <a
                href="#testimonials"
                className="text-gray-600 hover:text-black transition-colors"
              >
                Testimonials
              </a>
              <a
                href="#companies"
                className="text-gray-600 hover:text-black transition-colors"
              >
                Companies
              </a>
              <a
                href="#contact"
                className="text-gray-600 hover:text-black transition-colors"
              >
                Contact
              </a>
            </nav>
            <Button
              variant="outline"
              className="border-gray-300 text-black hover:bg-gray-50 bg-transparent"
            >
              Get Started
            </Button>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <section className="py-20 px-4 sm:px-6 lg:px-8 bg-black text-white">
        <div className="max-w-7xl mx-auto text-center">
          <Badge className="mb-6 bg-gray-800 text-white border-gray-700">
            🚀 Alpha version
          </Badge>
          <h1 className="text-5xl md:text-7xl font-bold mb-6 leading-tight">
            Load Testing
            <span className="text-gray-400"> Reimagined</span>
          </h1>
          <p className="text-xl text-gray-300 mb-8 max-w-3xl mx-auto leading-relaxed">
            V6 is the next-generation load testing framework that runs
            JavaScript and TypeScript. Built with Rust for unmatched
            performance, extensible architecture, and a rich API that beats the
            competition.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button
              size="lg"
              className="bg-white text-black hover:bg-gray-100 px-8 py-3"
            >
              Start Load Testing <ArrowRight className="ml-2 h-5 w-5" />
            </Button>
            <Button
              size="lg"
              variant="outline"
              className="border-white text-white hover:bg-white hover:text-black px-8 py-3 bg-transparent"
            >
              View Documentation
            </Button>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section id="features" className="py-20 px-4 sm:px-6 lg:px-8 bg-gray-50">
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-4xl font-bold text-black mb-4">
              Why Choose V6?
            </h2>
            <p className="text-xl text-gray-600 max-w-2xl mx-auto">
              Experience the future of load testing with cutting-edge technology
              and developer-first design.
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-8">
            <Card className="bg-white border-gray-200">
              <CardHeader>
                <Zap className="h-12 w-12 text-black mb-4" />
                <CardTitle className="text-black">Blazing Fast</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-gray-600">
                  Built on Rust's performance and V8 JavaScript engine, V6
                  delivers unparalleled speed and efficiency for your load
                  tests.
                </CardDescription>
              </CardContent>
            </Card>

            <Card className="bg-white border-gray-200">
              <CardHeader>
                <Globe className="h-12 w-12 text-black mb-4" />
                <CardTitle className="text-black">Web API Compatible</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-gray-600">
                  Full compatibility with modern Web APIs, enabling seamless
                  integration with existing web infrastructure and standards.
                </CardDescription>
              </CardContent>
            </Card>

            <Card className="bg-white border-gray-200">
              <CardHeader>
                <Code className="h-12 w-12 text-black mb-4" />
                <CardTitle className="text-black">Rich API</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-gray-600">
                  Comprehensive JavaScript/TypeScript API with intuitive methods
                  for complex load testing scenarios.
                </CardDescription>
              </CardContent>
            </Card>

            <Card className="bg-white border-gray-200">
              <CardHeader>
                <Users className="h-12 w-12 text-black mb-4" />
                <CardTitle className="text-black">Extensible</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-gray-600">
                  Plugin architecture and custom extensions let you tailor V6 to
                  your specific testing requirements.
                </CardDescription>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* Code Example */}
      <section className="py-20 px-4 sm:px-6 lg:px-8 bg-white">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold text-black mb-4">
              Simple. Powerful. Fast.
            </h2>
            <p className="text-xl text-gray-600">
              Get started with V6 in just a few lines of code
            </p>
          </div>

          <Card className="bg-black border-gray-800">
            <CardContent className="p-6">
              <pre className="text-green-400 font-mono text-sm overflow-x-auto">
                {`defineConfig({
  iterations: Infinity,
  vus: 10,
  duration: 30, // 30 seconds
  iteration: async function(){
    const response = await fetch('https://api.example.com/users');
    expect(response.ok).toBe(true);
  }
});`}
              </pre>
            </CardContent>
          </Card>
          <div className="mt-8 text-center">
            <p className="text-gray-600 mb-4">
              Save your test as{" "}
              <code className="bg-gray-100 px-2 py-1 rounded text-black font-mono">
                example.ts
              </code>{" "}
              and run it with:
            </p>
            <Card className="bg-gray-50 border-gray-200">
              <CardContent className="p-4">
                <code className="text-black font-mono text-lg">
                  v6 run ./example.ts
                </code>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* Companies Section */}
      <section id="companies" className="py-20 px-4 sm:px-6 lg:px-8 bg-gray-50">
        <div className="max-w-7xl mx-auto text-center">
          <h2 className="text-4xl font-bold text-black mb-4">
            Trusted by Growing Companies
          </h2>
          <p className="text-xl text-gray-600 mb-12">
            Join innovative companies using V6 for their load testing needs
          </p>

          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-8 items-center opacity-60">
            <div className="text-gray-700 font-bold text-2xl">Starbucks</div>
            <div className="text-gray-700 font-bold text-2xl">Nike</div>
            <div className="text-gray-700 font-bold text-2xl">Target</div>
            <div className="text-gray-700 font-bold text-2xl">McDonald's</div>
            <div className="text-gray-700 font-bold text-2xl">Walmart</div>
            <div className="text-gray-700 font-bold text-2xl">FedEx</div>
          </div>
        </div>
      </section>

      {/* Testimonials Section */}
      <section
        id="testimonials"
        className="py-20 px-4 sm:px-6 lg:px-8 bg-white"
      >
        <div className="max-w-7xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-4xl font-bold text-black mb-4">
              What Developers Say
            </h2>
            <p className="text-xl text-gray-600">
              Real feedback from teams using V6 in production
            </p>
          </div>

          <div className="grid md:grid-cols-3 gap-8">
            <Card className="bg-white border-gray-200">
              <CardContent className="p-6">
                <div className="flex mb-4">
                  {[...Array(5)].map((_, i) => (
                    <Star
                      key={i}
                      className="h-5 w-5 text-yellow-400 fill-current"
                    />
                  ))}
                </div>
                <p className="text-gray-600 mb-4">
                  "V6 completely transformed our load testing workflow. The
                  performance improvements over our previous solution were
                  immediately noticeable."
                </p>
                <div className="flex items-center">
                  <div className="w-10 h-10 bg-black rounded-full flex items-center justify-center text-white font-bold mr-3">
                    S
                  </div>
                  <div>
                    <p className="text-black font-semibold">Sarah Chen</p>
                    <p className="text-gray-500 text-sm">
                      Senior DevOps Engineer
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className="bg-white border-gray-200">
              <CardContent className="p-6">
                <div className="flex mb-4">
                  {[...Array(5)].map((_, i) => (
                    <Star
                      key={i}
                      className="h-5 w-5 text-yellow-400 fill-current"
                    />
                  ))}
                </div>
                <p className="text-gray-600 mb-4">
                  "The JavaScript/TypeScript support is fantastic. Our team was
                  productive from day one without learning a new language."
                </p>
                <div className="flex items-center">
                  <div className="w-10 h-10 bg-gray-600 rounded-full flex items-center justify-center text-white font-bold mr-3">
                    M
                  </div>
                  <div>
                    <p className="text-black font-semibold">Marcus Rodriguez</p>
                    <p className="text-gray-500 text-sm">
                      Lead Backend Developer
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className="bg-white border-gray-200">
              <CardContent className="p-6">
                <div className="flex mb-4">
                  {[...Array(5)].map((_, i) => (
                    <Star
                      key={i}
                      className="h-5 w-5 text-yellow-400 fill-current"
                    />
                  ))}
                </div>
                <p className="text-gray-600 mb-4">
                  "V6's extensibility allowed us to create custom plugins for
                  our specific use case. The architecture is brilliant."
                </p>
                <div className="flex items-center">
                  <div className="w-10 h-10 bg-gray-800 rounded-full flex items-center justify-center text-white font-bold mr-3">
                    A
                  </div>
                  <div>
                    <p className="text-black font-semibold">Alex Thompson</p>
                    <p className="text-gray-500 text-sm">
                      Performance Engineer
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* Contact Section */}
      {/*<section id="contact" className="py-20 px-4 sm:px-6 lg:px-8 bg-gray-50">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold text-black mb-4">Get in Touch</h2>
            <p className="text-xl text-gray-600">
              Ready to supercharge your load testing? Let's talk.
            </p>
          </div>

          <Card className="bg-white border-gray-200">
            <CardContent className="p-8">
              <form className="space-y-6">
                <div className="grid md:grid-cols-2 gap-6">
                  <div>
                    <label className="block text-black font-medium mb-2">
                      Name
                    </label>
                    <Input
                      className="bg-white border-gray-300 text-black placeholder:text-gray-400"
                      placeholder="Your name"
                    />
                  </div>
                  <div>
                    <label className="block text-black font-medium mb-2">
                      Email
                    </label>
                    <Input
                      type="email"
                      className="bg-white border-gray-300 text-black placeholder:text-gray-400"
                      placeholder="your@email.com"
                    />
                  </div>
                </div>
                <div>
                  <label className="block text-black font-medium mb-2">
                    Company
                  </label>
                  <Input
                    className="bg-white border-gray-300 text-black placeholder:text-gray-400"
                    placeholder="Your company"
                  />
                </div>
                <div>
                  <label className="block text-black font-medium mb-2">
                    Message
                  </label>
                  <Textarea
                    className="bg-white border-gray-300 text-black placeholder:text-gray-400 min-h-32"
                    placeholder="Tell us about your load testing needs..."
                  />
                </div>
                <Button className="w-full bg-black hover:bg-gray-800 text-white">
                  Send Message
                </Button>
              </form>
            </CardContent>
          </Card>
        </div>
      </section>*/}
      <iframe
        className="airtable-embed"
        src="https://airtable.com/embed/appmzLrh61IZwPobm/pagFtZMFu0VwQwV1x/form"
        width="100%"
        height="800"
        style={{ background: "transparent", border: "1px solid #ccc" }}
      ></iframe>

      {/* Footer */}
      <footer className="bg-black border-t border-gray-800 py-12 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto">
          <div className="grid md:grid-cols-4 gap-8">
            <div>
              <div className="flex items-center space-x-2 mb-4">
                <div className="w-8 h-8 bg-white rounded-lg flex items-center justify-center">
                  <span className="text-black font-bold text-lg">V6</span>
                </div>
                <span className="text-white font-bold text-xl">V6</span>
              </div>
              <p className="text-gray-400 mb-4">
                The next-generation load testing framework built for modern
                applications.
              </p>
              <div className="flex space-x-4">
                <Github className="h-5 w-5 text-gray-400 hover:text-white cursor-pointer transition-colors" >
                  <a href="https://github.com/vitalics/v6" target="_blank" rel="noopener noreferrer">
                  </a>
                </Github>
                {/*<Twitter className="h-5 w-5 text-gray-400 hover:text-white cursor-pointer transition-colors" />*/}
                <Linkedin className="h-5 w-5 text-gray-400 hover:text-white cursor-pointer transition-colors" />
              </div>
            </div>

            <div>
              <h3 className="text-white font-semibold mb-4">Product</h3>
              <ul className="space-y-2">
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Features
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Documentation
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    API Reference
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Examples
                  </a>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-white font-semibold mb-4">Company</h3>
              <ul className="space-y-2">
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    About
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Careers
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Blog
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Contact
                  </a>
                </li>
              </ul>
            </div>

            <div>
              <h3 className="text-white font-semibold mb-4">Legal</h3>
              <ul className="space-y-2">
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Privacy Policy
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Terms of Service
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Cookie Policy
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors"
                  >
                    Security
                  </a>
                </li>
              </ul>
            </div>
          </div>

          <div className="border-t border-gray-800 mt-8 pt-8 text-center">
            <p className="text-gray-400">
              © 2025 V6 Load Testing Framework. All rights reserved.
            </p>
          </div>
        </div>
      </footer>
    </div>
  );
}
